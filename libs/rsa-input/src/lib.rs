use rsa_core::settings::UPS;
use rsa_core::ty::Tag;
use std::collections::HashMap;
use std::ops::AddAssign;

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

	pub fn notify_event(&mut self, event: Event) {
		if event.kind.sustained() {
			if event.pressed {
				// If it already exists. End it here. (we prob missed the lift event)
				if let Some((press_action, mut state)) = self.active.remove(&event.kind) {
					state.fire_release = true;
					self.subtick_history.push(EventRecord::new_dual(
						press_action,
						event.clone(),
						state,
					));
				}

				self.active.insert(
					event.kind.clone(),
					(
						event.clone(),
						EventState {
							fire_press: true,
							fire_release: false,
						},
					),
				);
			} else {
				// Get the pressed action and calculate its time. Else make a 0 length event.
				if let Some((press_action, mut state)) = self.active.remove(&event.kind) {
					state.fire_release = true;
					self.subtick_history
						.push(EventRecord::new_dual(press_action, event, state));
				} else {
					self.subtick_history
						.push(EventRecord::new(event, EventState::full()));
				}
			}
		} else {
			self.subtick_history
				.push(EventRecord::new(event, EventState::full()));
		}
	}

	pub fn tick(&mut self) {
		for (kind, (event, state)) in self.active.iter_mut() {
			let delta = if let Some(time) = &mut event.start {
				let elapsed = time.elapsed();
				time.add_assign(elapsed);
				elapsed.as_secs_f64() / (1.0 / UPS as f64)
			} else {
				1.0
			};
			Self::invoke_subs(&self.bindings, &mut self.subscribers, kind, state, delta);
		}

		for action in &mut self.subtick_history {
			let tick_duration = action.tick_duration();
			Self::invoke_subs(
				&self.bindings,
				&mut self.subscribers,
				&action.kind,
				&mut action.state,
				tick_duration,
			);
		}

		self.subtick_history.clear();
	}

	fn invoke_subs(
		bindings: &HashMap<EventKind, Vec<Tag>>,
		subscribers: &mut HashMap<Tag, Vec<Subscriber>>,
		kind: &EventKind,
		state: &mut EventState,
		delta: f64,
	) {
		if let Some(subs) = bindings.get(kind) {
			for sub in subs {
				if let Some(subscribers) = subscribers.get_mut(sub) {
					for subscriber in subscribers {
						match subscriber {
							Subscriber::Hold { hold } => {
								hold(delta, 1.0);
							}
							Subscriber::Trigger { press, release } => {
								if state.fire_press {
									press(delta);
								}

								if state.fire_release {
									release(delta);
								}
							}
							Subscriber::Toggle { value, toggle } => {
								if state.fire_press {
									*value = !*value;
									toggle(delta, *value);
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
	use std::rc::Rc;
	use std::sync::atomic::{AtomicBool, Ordering};
	use std::sync::{Arc, Mutex};
	use std::time::Duration;

	use rsa_core::settings::UPS;

	use rsa_core::ty::{Direction, Tag};

	use crate::event::keyboard::{Key, KeyboardEvent};
	use crate::event::modifier::Modifiers;
	use crate::event::mouse::ScrollEvent;
	use crate::event::EventKind;
	use crate::{Event, EventRecord, EventState, InputSystem, Subscriber};

	const NSPU: u64 = (1000000000.0 / UPS as f64) as u64;

	pub fn keyboard() -> KeyboardEvent {
		KeyboardEvent {
			modifiers: Modifiers::empty(),
			key: Key::Char(' '),
		}
	}

	pub fn basic() -> EventKind {
		EventKind::Scroll(ScrollEvent {
			direction: Direction::Up,
		})
	}

	#[test]
	fn test_basic() {
		let mut input = InputSystem::new();
		input.notify_event(Event {
			kind: basic(),
			start: None,
			pressed: false,
		});

		assert_eq!(input.subtick_history[0].kind, basic())
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
		let mut input = InputSystem::new();
		let mut value = Rc::new(AtomicBool::new(false));

		let rc = value.clone();
		input.register_binding(Tag::rsa("tests"), vec![basic()]);
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::press(move |delta| {
				if rc.load(Ordering::Relaxed) {
					panic!("Double trigger")
				}
				rc.store(true, Ordering::Relaxed);
			}),
		);

		input.notify_event(Event {
			kind: basic(),
			start: None,
			pressed: true,
		});

		input.tick();

		if !value.load(Ordering::Relaxed) {
			panic!("Never triggered")
		}
	}

	#[test]
	fn test_toggle_subscriber() {
		let mut input = InputSystem::new();

		let value = Rc::new(AtomicBool::new(false));
		input.register_binding(Tag::rsa("tests"), vec![basic()]);

		let rc = value.clone();
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::toggle(false, move |delta, state| {
				rc.store(state, Ordering::Relaxed);
			}),
		);

		input.notify_event(Event {
			kind: basic(),
			start: None,
			pressed: true,
		});
		input.tick();

		// on
		assert!(value.load(Ordering::Relaxed));

		input.notify_event(Event {
			kind: basic(),
			start: None,
			pressed: true,
		});

		input.tick();

		// off
		assert!(!value.load(Ordering::Relaxed));
	}

	#[test]
	fn test_hold_subscriber() {
		let mut input = InputSystem::new();

		let value = Arc::new(Mutex::new(0.0));
		input.register_binding(Tag::rsa("tests"), vec![EventKind::Keyboard(keyboard())]);

		let valuee = value.clone();
		input.register_subscriber(
			Tag::rsa("tests"),
			Subscriber::hold(move |delta, value| {
				*valuee.lock().unwrap() += value * delta as f32;
			}),
		);

		// Lets wait a server tick, should be 1
		{
			input.notify_event(Event {
				kind: EventKind::Keyboard(keyboard()),
				start: Some(Instant::now()),
				pressed: true,
			});

			// +1 is a float moment
			MockClock::advance(Duration::from_nanos(NSPU + 1));
			input.tick();
			assert_eq!(*value.lock().unwrap(), 1.0);
		}

		// Lets go of the event half way through a tick
		{
			MockClock::advance(Duration::from_nanos(NSPU / 2));
			input.notify_event(Event {
				kind: EventKind::Keyboard(keyboard()),
				start: Some(Instant::now()),
				pressed: false,
			});
			input.tick();

			assert_eq!(*value.lock().unwrap(), 1.5);
		}
	}
}
