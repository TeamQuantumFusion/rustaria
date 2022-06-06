use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use glfw::{Action, Key, WindowEvent};

use rustaria::entity::component::physics::PhysicsComp;
use rustaria::player::Player;
use rsa_core::ty::Tag;
use rsa_core::settings::UPS;
use rsa_input::event::{Event, EventKind, KeyboardEvent};
use rsa_input::subscriber::{HoldSubscriber, Subscriber, TriggerSubscriber};
use rsa_input::InputSystem;

pub struct ControllerHandler {
	up: VelocitySubscriber,
	down: VelocitySubscriber,
	left: VelocitySubscriber,
	right: VelocitySubscriber,
	jump: JumpSubscriber,
	system: InputSystem,



	// player
	dir_x: f32,
	dir_y: f32,
	jump_frames_remaining: u32,
}

impl ControllerHandler {
	pub fn new() -> ControllerHandler {
		let up = VelocitySubscriber(Default::default());
		let down = VelocitySubscriber(Default::default());
		let left = VelocitySubscriber(Default::default());
		let right = VelocitySubscriber(Default::default());
		let jump = JumpSubscriber(Default::default());

		let mut input = InputSystem::new();
		input.register_binding(
			Tag::rsa("up"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::W))],
		);
		input.register_binding(
			Tag::rsa("down"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::S))],
		);
		input.register_binding(
			Tag::rsa("left"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::A))],
		);
		input.register_binding(
			Tag::rsa("right"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::D))],
		);
		input.register_binding(
			Tag::rsa("jump"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::Space))],
		);

		input.register_subscriber(Tag::rsa("up"), Subscriber::Hold(Box::new(up.clone())));
		input.register_subscriber(
			Tag::rsa("down"),
			Subscriber::Hold(Box::new(down.clone())),
		);
		input.register_subscriber(
			Tag::rsa("left"),
			Subscriber::Hold(Box::new(left.clone())),
		);
		input.register_subscriber(
			Tag::rsa("right"),
			Subscriber::Hold(Box::new(right.clone())),
		);
		input.register_subscriber(
			Tag::rsa("jump"),
			Subscriber::Trigger(Box::new(jump.clone())),
		);

		ControllerHandler {
			up,
			down,
			left,
			right,
			jump,
			system: input,
			dir_x: 0.0,
			dir_y: 0.0,
			jump_frames_remaining: 0,
		}
	}

	pub fn consume_event(&mut self, event: WindowEvent) {
		let event = match event {
			WindowEvent::Key(key, _, value @ (Action::Press | Action::Release), modifier) => {
				Event {
					kind: EventKind::Keyboard(KeyboardEvent { key, modifier }),
					start: Some(Instant::now()),
					pressed: value == Action::Press,
				}
			}
			_ => {
				return;
			}
		};
		self.system.notify_event(event);
	}

	pub fn apply(&mut self, physics: &mut PhysicsComp, touches_ground: bool, player: &Player) {
		self.system.tick();

		physics.velocity.x += (*self.right.0.lock().unwrap() - *self.left.0.lock().unwrap())
			* (player.run_acceleration / UPS as f32);
		physics.velocity.y += (*self.up.0.lock().unwrap() - *self.down.0.lock().unwrap())
			* (player.run_acceleration / UPS as f32);
		self.dir_x = 0.0;
		self.dir_y = 0.0;

		let jumped = self.jump.0.load(Ordering::Relaxed);
		if physics.velocity.x > player.run_max_speed / UPS as f32 {
			physics.velocity.x = player.run_max_speed / UPS as f32;
		} else if physics.velocity.x < -(player.run_max_speed / UPS as f32) {
			physics.velocity.x = -player.run_max_speed / UPS as f32;
		}

		if touches_ground {
			if physics.velocity.x > player.run_slowdown / UPS as f32 {
				physics.velocity.x -= player.run_slowdown / UPS as f32;
			} else if physics.velocity.x < -(player.run_slowdown / UPS as f32) {
				physics.velocity.x += player.run_slowdown / UPS as f32;
			} else {
				physics.velocity.x = 0.0;
			}

			if jumped {
				self.jump_frames_remaining = player.jump_frames;
			}
		}

		if self.jump_frames_remaining > 0 {
			if jumped {
				physics.velocity.y = player.jump_speed / UPS as f32;
				self.jump_frames_remaining -= 1;
			} else {
				self.jump_frames_remaining = 0;
			}
		}

		*self.up.0.lock().unwrap() = 0.0;
		*self.down.0.lock().unwrap() = 0.0;
		*self.left.0.lock().unwrap() = 0.0;
		*self.right.0.lock().unwrap() = 0.0;
	}
}

#[derive(Clone)]
struct VelocitySubscriber(Arc<Mutex<f32>>);

impl HoldSubscriber for VelocitySubscriber {
	fn hold(&mut self, delta: f32) {
		println!("Hold {delta}");
		*self.0.lock().unwrap() += delta;
	}
}

#[derive(Clone)]
struct JumpSubscriber(Arc<AtomicBool>);

impl TriggerSubscriber for JumpSubscriber {
	fn pressed(&mut self) {
		self.0.store(true, Ordering::Relaxed);
	}

	fn released(&mut self) {
		self.0.store(false, Ordering::Relaxed);
	}
}
