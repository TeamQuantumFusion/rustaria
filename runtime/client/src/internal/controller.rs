use std::collections::HashMap;

use glfw::{Key, WindowEvent};

use rustaria::entity::component::velocity::PhysicsComp;
use rustaria::player::Player;
use rustaria::UPS;
use rustaria_controller::button::{ButtonKey, HoldSubscriber, TriggerSubscriber};
use rustaria_controller::Controller;
use rustariac_backend::ty::Camera;

// TODO remake this
pub(crate) struct ControllerHandler {
	up: HoldSubscriber,
	down: HoldSubscriber,
	left: HoldSubscriber,
	right: HoldSubscriber,
	jump: HoldSubscriber,
	controller: Controller,
	old_delta: f32,

	// player
	dir_x: f32,
	dir_y: f32,
	jump_frames_remaining: u32,
}

impl ControllerHandler {
	pub fn new() -> ControllerHandler {
		let mut bindings = HashMap::new();
		bindings.insert("up".to_string(), ButtonKey::Keyboard(Key::W));
		bindings.insert("down".to_string(), ButtonKey::Keyboard(Key::S));
		bindings.insert("left".to_string(), ButtonKey::Keyboard(Key::A));
		bindings.insert("right".to_string(), ButtonKey::Keyboard(Key::D));
		bindings.insert("jump".to_string(), ButtonKey::Keyboard(Key::Space));

		let up = HoldSubscriber::new();
		let down = HoldSubscriber::new();
		let left = HoldSubscriber::new();
		let right = HoldSubscriber::new();
		let zoom_in = TriggerSubscriber::new();
		let zoom_out = TriggerSubscriber::new();
		let jump = HoldSubscriber::new();

		let mut controller = Controller::new(bindings);
		controller.subscribe(Box::new(up.clone()), "up".to_string());
		controller.subscribe(Box::new(down.clone()), "down".to_string());
		controller.subscribe(Box::new(left.clone()), "left".to_string());
		controller.subscribe(Box::new(right.clone()), "right".to_string());
		controller.subscribe(Box::new(jump.clone()), "jump".to_string());
		controller.apply();

		ControllerHandler {
			up,
			down,
			left,
			right,
			jump,
			controller,
			old_delta: 0.0,
			dir_x: 0.0,
			dir_y: 0.0,
			jump_frames_remaining: 0,
		}
	}

	pub fn consume_event(&mut self, event: WindowEvent) {
		self.controller.consume(event);
	}

	pub fn apply(&mut self, physics: &mut PhysicsComp, touches_ground: bool, player: &Player) {
		physics.velocity.x += self.dir_x.clamp(-1.0, 1.0) * (player.run_acceleration / UPS as f32);
		physics.velocity.y += self.dir_y.clamp(-1.0, 1.0) * (player.run_acceleration / UPS as f32);
		self.dir_x = 0.0;
		self.dir_y = 0.0;

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

			if self.jump.held() {
				self.jump_frames_remaining = player.jump_frames;
			}
		}

		if self.jump_frames_remaining > 0 && self.jump.held() {
			physics.velocity.y = player.jump_speed / UPS as f32;
			self.jump_frames_remaining -= 1;
		}
	}

	pub fn draw(&mut self, view: &mut Camera, delta: f32) {
		if self.old_delta > delta {
			self.old_delta = delta;
		}

		// trademark this:tm:
		let delta_delta_tm = delta - self.old_delta;
		if self.up.held() {
			self.dir_y += delta_delta_tm;
		}
		if self.down.held() {
			self.dir_y -= delta_delta_tm;
		}
		if self.right.held() {
			self.dir_x += delta_delta_tm;
		}
		if self.left.held() {
			self.dir_x -= delta_delta_tm;
		}

		self.old_delta = delta;
	}
}
