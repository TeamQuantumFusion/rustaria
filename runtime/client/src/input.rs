use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use rsa_core::logging::info;
use rsa_core::ty::{Direction, Tag};
use rsa_input::event::mouse::ScrollEvent;
use rsa_input::event::EventKind;
use rsa_input::InputSystem;
use rsa_input::subscriber::Subscriber;
use rsac_graphic::camera::Camera;

pub struct InputModule {
	pub system: InputSystem,
	zoom_in: Rc<AtomicU32>,
	zoom_out: Rc<AtomicU32>,
}

impl InputModule {
	pub fn new() -> InputModule {
		info!(target: "init@rustaria", "Initializing Inputs");
		let mut input = InputSystem::new();

		let zoom_in = Rc::new(AtomicU32::new(0));
		input.register_binding(
			Tag::rsa("zoom_in"),
			vec![EventKind::Scroll(ScrollEvent {
				direction: Direction::Up,
			})],
		);

		let rc = zoom_in.clone();
		input.register_subscriber(Tag::rsa("zoom_in"), Subscriber::press(move || {
			rc.fetch_add(1, Ordering::Relaxed);
		}));

		let zoom_out = Rc::new(AtomicU32::new(0));
		input.register_binding(
			Tag::rsa("zoom_out"),
			vec![EventKind::Scroll(ScrollEvent {
				direction: Direction::Down,
			})],
		);

		let rc = zoom_out.clone();
		input.register_subscriber(Tag::rsa("zoom_out"), Subscriber::press(move || {
			rc.fetch_add(1, Ordering::Relaxed);
		}));
		InputModule { system: input, zoom_in, zoom_out }
	}

	pub fn tick(&mut self) {
		self.system.tick();
	}

	pub fn apply_zoom(&mut self, camera: &mut Camera) {
		let amount = (self.zoom_out.load(Ordering::Relaxed) as f32 - self.zoom_in.load(Ordering::Relaxed) as f32) / 1.0;
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
