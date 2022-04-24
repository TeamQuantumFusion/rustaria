use std::collections::HashMap;

use glfw::{Key, WindowEvent};
use rustaria::entity::component::velocity::PhysicsComp;
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
	jump: TriggerSubscriber,
	zoom_in: TriggerSubscriber,
	zoom_out: TriggerSubscriber,
	controller: Controller,
	old_delta: f32,
}

impl ControllerHandler {
	pub fn new() -> ControllerHandler {
		let mut bindings = HashMap::new();
		bindings.insert("up".to_string(), ButtonKey::Keyboard(Key::W));
		bindings.insert("down".to_string(), ButtonKey::Keyboard(Key::S));
		bindings.insert("left".to_string(), ButtonKey::Keyboard(Key::A));
		bindings.insert("right".to_string(), ButtonKey::Keyboard(Key::D));
		bindings.insert("zoom_in".to_string(), ButtonKey::Keyboard(Key::R));
		bindings.insert("zoom_out".to_string(), ButtonKey::Keyboard(Key::F));
		bindings.insert("jump".to_string(), ButtonKey::Keyboard(Key::Space));

		let up = HoldSubscriber::new();
		let down = HoldSubscriber::new();
		let left = HoldSubscriber::new();
		let right = HoldSubscriber::new();
		let zoom_in = TriggerSubscriber::new();
		let zoom_out = TriggerSubscriber::new();
		let jump = TriggerSubscriber::new();

		let mut controller = Controller::new(bindings);
		controller.subscribe(Box::new(up.clone()), "up".to_string());
		controller.subscribe(Box::new(down.clone()), "down".to_string());
		controller.subscribe(Box::new(left.clone()), "left".to_string());
		controller.subscribe(Box::new(right.clone()), "right".to_string());
		controller.subscribe(Box::new(zoom_in.clone()), "zoom_in".to_string());
		controller.subscribe(Box::new(zoom_out.clone()), "zoom_out".to_string());
		controller.subscribe(Box::new(jump.clone()), "jump".to_string());
		controller.apply();

		ControllerHandler {
			up,
			down,
			left,
			right,
			jump,
			zoom_in,
			zoom_out,
			controller,
			old_delta: 0.0,
		}
	}

	pub fn consume_event(&mut self, event: WindowEvent) {
		self.controller.consume(event);
	}

	pub fn tick(&mut self, physics: &mut PhysicsComp) {
		const SPEED: f32 = 0.1;
		if self.up.held() && physics.acceleration.y < 2.0 {
			physics.acceleration.y += SPEED;
		}
		if self.down.held() && physics.acceleration.y > -2.0 {
			physics.acceleration.y -= SPEED;
		}
		if self.right.held() && physics.acceleration.x < 2.0 {
			physics.acceleration.x += SPEED;
		}

		if self.left.held() && physics.acceleration.x > -2.0 {
			physics.acceleration.x -= SPEED;
		}

		if self.jump.triggered() {
			physics.acceleration.y += 6.0;
		}
	}

	pub fn draw(&mut self, view: &mut Camera, delta: f32) {
		if self.old_delta > delta {
			self.old_delta = delta;
		}

		let movement_delta = delta - self.old_delta;
		let zoom = view.zoom / 30.0;
		view.velocity = [0.0, 0.0];
		if self.up.held() {
			view.velocity[1] += 6.0 * movement_delta * zoom;
		}
		if self.down.held() {
			view.velocity[1] -= 6.0 * movement_delta * zoom;
		}
		if self.left.held() {
			view.velocity[0] -= 6.0 * movement_delta * zoom;
		}
		if self.right.held() {
			view.velocity[0] += 6.0 * movement_delta * zoom;
		}
		if self.zoom_in.triggered() {
			view.zoom += 5.0;
		}
		if self.zoom_out.triggered() {
			view.zoom -= 5.0;
		}
		self.old_delta = delta;
	}
}
