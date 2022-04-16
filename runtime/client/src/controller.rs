use glfw::Modifiers;
use glfw::{Action, Key, WindowEvent};
use std::collections::HashMap;

use rustaria_controller::button::{ButtonKey, ButtonSubscriber, HoldSubscriber, TriggerSubscriber};
use rustaria_controller::Controller;
use rustaria_util::info;
use rustariac_backend::ty::Viewport;

pub struct PrintSubscriber {
	pub message: &'static str,
}

impl ButtonSubscriber for PrintSubscriber {
	fn event(&mut self, action: Action, modifiers: Modifiers) {
		info!("{} {:?} {:?}", self.message, action, modifiers);
	}
}

pub(crate) struct ControllerHandler {
	up: HoldSubscriber,
	down: HoldSubscriber,
	left: HoldSubscriber,
	right: HoldSubscriber,
	zoom_in: TriggerSubscriber,
	zoom_out: TriggerSubscriber,
	controller: Controller,
    old_delta: f32
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

		let up = HoldSubscriber::new();
		let down = HoldSubscriber::new();
		let left = HoldSubscriber::new();
		let right = HoldSubscriber::new();
		let zoom_in = TriggerSubscriber::new();
		let zoom_out = TriggerSubscriber::new();

		let mut controller = Controller::new(bindings);
		controller.subscribe(Box::new(up.clone()), "up".to_string());
		controller.subscribe(Box::new(down.clone()), "down".to_string());
		controller.subscribe(Box::new(left.clone()), "left".to_string());
		controller.subscribe(Box::new(right.clone()), "right".to_string());
		controller.subscribe(Box::new(zoom_in.clone()), "zoom_in".to_string());
		controller.subscribe(Box::new(zoom_out.clone()), "zoom_out".to_string());
		controller.apply();

		ControllerHandler {
			up,
			down,
			left,
			right,
			zoom_in,
			zoom_out,
			controller,
		old_delta: 0.0}
	}

	pub fn consume_event(&mut self, event: WindowEvent) {
		self.controller.consume(event);
	}

	pub fn tick(&mut self, view: &mut Viewport, delta: f32) {
		if self.old_delta > delta {
            self.old_delta = delta;
        }

        let movement_delta = delta - self.old_delta;let zoom = view.zoom / 30.0;
		if self.up.held() {
			view.position[1] += 1.6 * movement_delta * zoom;
		}
		if self.down.held() {
			view.position[1] -= 1.6 * movement_delta * zoom;
		}
		if self.left.held() {
			view.position[0] -= 1.6 * movement_delta * zoom;
		}
		if self.right.held() {
			view.position[0] += 1.6 * movement_delta * zoom;
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
