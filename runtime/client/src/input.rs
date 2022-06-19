use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use rsa_core::error::Result;
use rsa_core::logging::info;
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::{Direction, Tag};
use rsa_input::event::keyboard::Key;
use rsa_input::event::mouse::ScrollEvent;
use rsa_input::event::{Event, EventKind};
use rsa_input::subscriber::Subscriber;
use rsa_input::InputSystem;
use rsac_graphic::camera::Camera;
use crate::{PlayerModule, vec2};

pub struct InputModule {
	pub system: InputSystem,
	zoom_in: Rc<AtomicU32>,
	zoom_out: Rc<AtomicU32>,
	vec: Rc<Mutex<Vector2D<f32, WorldSpace>>>,
}

impl InputModule {
	pub fn new() -> InputModule {
		info!(target: "init@rustaria", "Initializing Inputs");
		let mut input = InputSystem::new();

		let zoom_in = Self::counter(
			&mut input,
			"zoom_in",
			EventKind::Scroll(ScrollEvent {
				direction: Direction::Up,
			}),
		);
		let zoom_out = Self::counter(
			&mut input,
			"zoom_out",
			EventKind::Scroll(ScrollEvent {
				direction: Direction::Down,
			}),
		);

		input.register_binding(Tag::rsa("up"), vec![EventKind::key(Key::Char('w'))]);
		input.register_binding(Tag::rsa("down"), vec![EventKind::key(Key::Char('s'))]);
		input.register_binding(Tag::rsa("left"), vec![EventKind::key(Key::Char('a'))]);
		input.register_binding(Tag::rsa("right"), vec![EventKind::key(Key::Char('d'))]);

		let dir = Rc::new(Mutex::new(Vector2D::new(0.0, 0.0)));

		let rc = dir.clone();
		input.register_subscriber(
			Tag::rsa("up"),
			Subscriber::hold(move |delta, value| rc.lock().unwrap().y += value * delta as f32),
		);
		let rc = dir.clone();
		input.register_subscriber(
			Tag::rsa("down"),
			Subscriber::hold(move |delta, value| rc.lock().unwrap().y -= value * delta as f32),
		);
		let rc = dir.clone();
		input.register_subscriber(
			Tag::rsa("left"),
			Subscriber::hold(move |delta, value| rc.lock().unwrap().x -= value * delta as f32),
		);
		let rc = dir.clone();
		input.register_subscriber(
			Tag::rsa("right"),
			Subscriber::hold(move |delta, value| rc.lock().unwrap().x += value * delta as f32),
		);

		InputModule {
			system: input,
			zoom_in,
			zoom_out,
			vec: dir,
		}
	}

	fn counter(input: &mut InputSystem, name: &'static str, event: EventKind) -> Rc<AtomicU32> {
		let value = Rc::new(AtomicU32::new(0));
		input.register_binding(Tag::rsa(name), vec![event]);

		let rc = value.clone();
		input.register_subscriber(
			Tag::rsa(name),
			Subscriber::press(move |_| {
				rc.fetch_add(1, Ordering::Relaxed);
			}),
		);
		value
	}

	pub fn tick_input(&mut self, new_events: Vec<Event>) {
		for event in new_events {
			self.system.notify_event(event);
		}
		self.system.tick();
	}

	pub fn apply_movement(&mut self, player: &mut PlayerModule) -> Result<()> {
		let mut guard = self.vec.lock().unwrap();
		player.set_movement_direction(*guard);
		*guard = vec2(0.0, 0.0);
		Ok(())
	}

	// Update camera
	pub fn setup_camera(&mut self, camera: &mut Camera) {
		// TODO prediction
		// Zoom
		let amount = (self.zoom_out.load(Ordering::Relaxed) as f32
			- self.zoom_in.load(Ordering::Relaxed) as f32)
			/ 1.0;
		if amount != 0.0 {
			// Apply a bit of scaling to make zoom more natural,
			// if you are wondering how i came up with these numbers il tell you my secret.
			// I just put in numbers until it works.
			camera.scale += amount * ((camera.scale / 15.0) + 0.5);
			camera.scale = camera.scale.clamp(5.0, 50.0);
		}
		self.zoom_out.store(0, Ordering::Relaxed);
		self.zoom_in.store(0, Ordering::Relaxed);
	}
}
