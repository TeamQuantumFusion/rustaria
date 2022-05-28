use std::collections::HashMap;
use rsa_core::ty::Tag;

use crate::event::{Event, EventKind, EventRecord, EventState};
use crate::subscriber::Subscriber;

pub mod event;
pub mod subscriber;

pub struct InputSystem {
	// cannot be reset
	active: HashMap<EventKind, (Event, EventState)>,
	// will be reset every tick
	subtick_history: Vec<EventRecord>,

	bindings: HashMap<EventKind, Vec<Tag>>,
	subscribers: HashMap<Tag, Vec<Subscriber>>,
}

impl InputSystem {
	pub fn new() -> InputSystem {
		InputSystem {
			active: Default::default(),
			subtick_history: vec![],
			bindings: Default::default(),
			subscribers: Default::default(),
		}
	}

	pub fn register_subscriber(&mut self, tag: Tag, subscriber: Subscriber) {
		self.subscribers
			.entry(tag)
			.or_insert_with(Vec::new)
			.push(subscriber);
	}

	pub fn register_binding(&mut self, tag: Tag, events: Vec<EventKind>) {
		for event in events {
			self.bindings
				.entry(event)
				.or_insert_with(Vec::new)
				.push(tag.clone());
		}
	}

	pub fn notify_event(&mut self, action: Event) {
		if action.kind.sustained() {
			if action.pressed {
				// If it already exists. End it here. (we prob missed the lift event)
				if let Some((press_action, mut state)) = self.active.remove(&action.kind) {
					state.fire_release = true;
					self.subtick_history.push(EventRecord::new_dual(
						press_action,
						action.clone(),
						state,
					));
				}

				self.active.insert(
					action.kind.clone(),
					(
						action.clone(),
						EventState {
							fire_press: true,
							fire_release: false,
						},
					),
				);
			} else {
				// Get the pressed action and calculate its time. Else make a 0 length event.
				if let Some((press_action, mut state)) = self.active.remove(&action.kind) {
					state.fire_release = true;
					self.subtick_history
						.push(EventRecord::new_dual(press_action, action, state));
				} else {
					self.subtick_history
						.push(EventRecord::new(action, EventState::full()));
				}
			}
		} else {
			self.subtick_history
				.push(EventRecord::new(action, EventState::full()));
		}
	}

	pub fn tick(&mut self) {
		for (kind, (_, state)) in self.active.iter_mut() {
			Self::invoke_subs(&self.bindings, &mut self.subscribers, kind, state, 1.0);
		}

		for action in &mut self.subtick_history {
			let delta = action.get_delta();
			Self::invoke_subs(
				&self.bindings,
				&mut self.subscribers,
				&action.kind,
				&mut action.state,
				delta,
			);
		}

		self.subtick_history.clear();
	}

	fn invoke_subs(
		bindings: &HashMap<EventKind, Vec<Tag>>,
		subscribers: &mut HashMap<Tag, Vec<Subscriber>>,
		kind: &EventKind,
		state: &mut EventState,
		delta: f32,
	) {
		if let Some(subs) = bindings.get(kind) {
			for sub in subs {
				if let Some(subscribers) = subscribers.get_mut(sub) {
					for subscriber in subscribers {
						match subscriber {
							Subscriber::Hold(value) => {
								value.hold(delta);
							}
							Subscriber::Trigger(value) => {
								if state.fire_press {
									value.pressed();
								}

								if state.fire_release {
									value.released();
								}
							}
							Subscriber::Toggle(value, switch) => {
								if state.fire_press {
									*switch = !*switch;
									value.toggle(*switch);
								}
							}
						}
					}
				}
			}
		}

		state.fire_press = false;
		state.fire_release = false;
	}
}
#[cfg(test)]
mod tests {
	use mock_instant::{Instant, MockClock};
	use std::sync::atomic::{AtomicBool, Ordering};
	use std::sync::{Arc, Mutex};
	use std::time::Duration;

	use glfw::{Key, Modifiers};
	use rustaria_api::ty::Tag;
	use rsa_core::settings::UPS;

	use rsa_core::ty::{Direction, Tag};

	use crate::event::{EventKind, KeyboardEvent};
	use crate::subscriber::{HoldSubscriber, ToggleSubscriber, TriggerSubscriber};
	use crate::{Event, EventRecord, EventState, InputSystem, Subscriber};

	const NSPU: u64 = (1000000000.0 / UPS as f64) as u64;

	pub fn keyboard() -> KeyboardEvent {
		KeyboardEvent {
			key: Key::Space,
			modifier: Modifiers::empty(),
		}
	}

	#[test]
	fn test_basic() {
		let mut input = InputSystem::new();
		input.notify_event(Event {
			kind: EventKind::Scroll(Direction::Up),
			start: None,
			pressed: false,
		});

		assert_eq!(
			input.subtick_history[0].kind,
			EventKind::Scroll(Direction::Up)
		)
	}

	#[test]
	fn test_sustained() {
		let mut input = InputSystem::new();

		// press
		input.notify_event(Event {
			kind: EventKind::Keyboard(keyboard()),
			start: Some(Instant::now()),
			pressed: true,
		});

		MockClock::advance(Duration::from_secs(10));

		// release
		input.notify_event(Event {
			kind: EventKind::Keyboard(keyboard()),
			start: Some(Instant::now()),
			pressed: false,
		});

		let record = &input.subtick_history[0];
		match &record.kind {
			EventKind::Keyboard(_) => {
				// If this entire operation takes a whole second more i have no words.
				assert_eq!(
					record.duration.expect("Could not find duration.").as_secs(),
					10
				);
			}
			_ => panic!("Wrong type."),
		}
	}

	#[test]
	fn test_missed_release() {
		let kind = EventKind::Keyboard(keyboard());

		let mut input = InputSystem::new();

		// press
		let start_time = Instant::now();
		input.notify_event(Event {
			kind: kind.clone(),
			start: Some(start_time),
			pressed: true,
		});

		input.tick();
		MockClock::advance(Duration::from_secs(10));

		// Press again and see if it handles the "missed" event correctly
		let end_time = Instant::now();
		input.notify_event(Event {
			kind: kind.clone(),
			start: Some(end_time),
			pressed: true,
		});

		let duration = end_time.saturating_duration_since(start_time);
		assert_eq!(input.subtick_history.len(), 1);
		// Ensure the missed event got tracked in history
		assert_eq!(
			input.subtick_history[0],
			EventRecord {
				kind: kind.clone(),
				duration: Some(duration),
				state: EventState {
					fire_press: false,
					fire_release: true
				}
			}
		);
		// Ensure the press is there.
		assert_eq!(
			*input.active.get(&kind).unwrap(),
			(
				Event {
					kind,
					start: Some(end_time),
					pressed: true
				},
				EventState {
					fire_press: true,
					fire_release: false
				}
			)
		);
	}

	#[test]
	fn test_missed_press() {
		let kind = EventKind::Keyboard(keyboard());
		let mut input = InputSystem::new();

		let time = Instant::now();
		input.notify_event(Event {
			kind: kind.clone(),
			start: Some(time),
			// release
			pressed: false,
		});

		assert_eq!(input.active.len(), 0);
		assert_eq!(
			input.subtick_history[0],
			EventRecord {
				kind,
				duration: None,
				state: EventState::full()
			}
		);
	}

	#[test]
	fn test_trigger_subscriber() {
		#[derive(Clone)]
		pub struct Sub(Arc<AtomicBool>);
		impl TriggerSubscriber for Sub {
			fn pressed(&mut self) {
				if self.0.load(Ordering::Relaxed) {
					panic!("Double trigger")
				}
				self.0.store(true, Ordering::Relaxed);
			}

			fn released(&mut self) {}
		}

		let sub = Sub(Default::default());

		let mut input = InputSystem::new();
		input.register_binding(Tag::rsa("tests"), vec![EventKind::Scroll(Direction::Up)]);
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::Trigger(Box::new(sub.clone())),
		);

		input.notify_event(Event {
			kind: EventKind::Scroll(Direction::Up),
			start: None,
			pressed: true,
		});

		input.tick();

		if !sub.0.load(Ordering::Relaxed) {
			panic!("Never triggered")
		}
	}

	#[test]
	fn test_toggle_subscriber() {
		#[derive(Clone)]
		pub struct Sub(Arc<AtomicBool>);
		impl ToggleSubscriber for Sub {
			fn toggle(&mut self, state: bool) {
				self.0.store(state, Ordering::Relaxed);
			}
		}

		let sub = Sub(Default::default());

		let mut input = InputSystem::new();
		input.register_binding(Tag::rsa("tests"), vec![EventKind::Scroll(Direction::Up)]);
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::Toggle(Box::new(sub.clone()), false),
		);

		input.notify_event(Event {
			kind: EventKind::Scroll(Direction::Up),
			start: None,
			pressed: true,
		});
		input.tick();

		// on
		assert!(sub.0.load(Ordering::Relaxed));

		input.notify_event(Event {
			kind: EventKind::Scroll(Direction::Up),
			start: None,
			pressed: true,
		});

		input.tick();

		// off
		assert!(!sub.0.load(Ordering::Relaxed));
	}

	#[test]
	fn test_hold_subscriber() {
		#[derive(Clone)]
		pub struct Sub(Arc<Mutex<f32>>);
		impl HoldSubscriber for Sub {
			fn hold(&mut self, delta: f32) {
				*self.0.lock().unwrap() += delta;
			}
		}

		let sub = Sub(Default::default());

		let mut input = InputSystem::new();
		input.register_binding(
			Tag::rsa("tests"),
			vec![EventKind::Keyboard(KeyboardEvent {
				key: Key::Space,
				modifier: Modifiers::empty(),
			})],
		);
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::Hold(Box::new(sub.clone())),
		);

		// Holds across ticks. Should be 1.0
		{
			input.notify_event(Event {
				kind: EventKind::Keyboard(keyboard()),
				start: Some(Instant::now()),
				pressed: true,
			});

			input.tick();
			assert_eq!(*sub.0.lock().unwrap(), 1.0);
		}

		MockClock::advance(Duration::from_nanos(NSPU));

		// Lets go of the event half way through a tick
		{
			MockClock::advance(Duration::from_nanos(NSPU / 2));
			input.notify_event(Event {
				kind: EventKind::Keyboard(keyboard()),
				start: Some(Instant::now()),
				pressed: false,
			});
			input.tick();

			assert_eq!(*sub.0.lock().unwrap(), 1.5);
		}
	}
}
