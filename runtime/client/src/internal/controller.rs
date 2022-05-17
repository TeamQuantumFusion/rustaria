use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Instant;

use glfw::{Action, Key, WindowEvent};

use rustaria::entity::component::velocity::PhysicsComp;
use rustaria::player::Player;
use rustaria_api::ty::Tag;
use rustaria_common::logging::{error, info};
use rustaria_common::settings::UPS;
use rustaria_input::event::{Event, EventKind, KeyboardEvent};
use rustaria_input::InputSystem;
use rustaria_input::subscriber::{HoldSubscriber, Subscriber, TriggerSubscriber};
use rustariac_backend::ty::Camera;

// TODO remake this
pub(crate) struct ControllerHandler {
	up: VelocitySubscriber,
	down: VelocitySubscriber,
	left: VelocitySubscriber,
	right: VelocitySubscriber,
	jump: JumpSubscriber,
	system: InputSystem,
	old_delta: f32,

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
			Tag::builtin("up"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::W))],
		);
		input.register_binding(
			Tag::builtin("down"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::S))],
		);
		input.register_binding(
			Tag::builtin("left"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::A))],
		);
		input.register_binding(
			Tag::builtin("right"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::D))],
		);
		input.register_binding(
			Tag::builtin("jump"),
			vec![EventKind::Keyboard(KeyboardEvent::new(Key::Space))],
		);

		input.register_subscriber(Tag::builtin("up"), Subscriber::Hold(Box::new(up.clone())));
		input.register_subscriber(Tag::builtin("down"), Subscriber::Hold(Box::new(down.clone())));
		input.register_subscriber(Tag::builtin("left"), Subscriber::Hold(Box::new(left.clone())));
		input.register_subscriber(Tag::builtin("right"), Subscriber::Hold(Box::new(right.clone())));
		input.register_subscriber(Tag::builtin("jump"), Subscriber::Trigger(Box::new(jump.clone())));

		ControllerHandler {
			up,
			down,
			left,
			right,
			jump,
			system: input,
			old_delta: 0.0,
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

		physics.velocity.x += (*self.right.0.lock().unwrap() - *self.left.0.lock().unwrap()) * (player.run_acceleration / UPS as f32);
		physics.velocity.y += (*self.up.0.lock().unwrap() - *self.down.0.lock().unwrap()) * (player.run_acceleration / UPS as f32);
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

		if self.jump_frames_remaining > 0  {
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

	pub fn draw(&mut self, view: &mut Camera, delta: f32) {
		if self.old_delta > delta {
			self.old_delta = delta;
		}

		// trademark this:tm:
		//let delta_delta_tm = delta - self.old_delta;
		//if self.up.held() {
		//	self.dir_y += delta_delta_tm;
		//}
		//if self.down.held() {
		//	self.dir_y -= delta_delta_tm;
		//}
		//if self.right.held() {
		//	self.dir_x += delta_delta_tm;
		//}
		//if self.left.held() {
		//	self.dir_x -= delta_delta_tm;
		//}

		self.old_delta = delta;
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