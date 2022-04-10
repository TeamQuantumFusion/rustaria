use std::collections::{HashMap, HashSet};

use glfw::WindowEvent;

use rustaria_util::warn;

use crate::button::{ButtonKey, ButtonSubscriber};

pub mod button;

pub type SubscriberId = u64;

pub struct ControllerHandler {
	// this gets compiled.
	button_lookup: HashMap<ButtonKey, HashSet<SubscriberId>>,
	button_name_lookup: HashMap<String, HashSet<SubscriberId>>,
	button_bindings: HashMap<String, ButtonKey>,
	button_subscribers: Vec<Box<dyn ButtonSubscriber>>,
	id: SubscriberId
}

impl ControllerHandler {
	pub fn new(bindings: HashMap<String, ButtonKey>) -> ControllerHandler {
		ControllerHandler {
			button_lookup: Default::default(),
			button_name_lookup: Default::default(),
			button_bindings: bindings,
			button_subscribers: vec![],
			id: 0
		}
	}

	pub fn subscribe(&mut self, subscriber: Box<dyn ButtonSubscriber>, name: String) {
		self.button_subscribers.insert(self.id as usize, subscriber);
		self.button_name_lookup.entry(name.clone()).or_insert_with(HashSet::new);
		self.button_name_lookup.get_mut(&name).unwrap().insert(self.id);
		self.id += 1;
	}

	pub fn apply(&mut self) {
		for (name, key) in &self.button_bindings {
			if let Some(ids) = self.button_name_lookup.get(name) {
				self.button_lookup.entry(*key).or_insert_with(HashSet::new);
				let lookup = self.button_lookup.get_mut(key).unwrap();
				for id in ids {
					lookup.insert(*id);
				}
			} else {
				warn!("Could not find subscribers for {}", name)
			}
		}
	}

	pub fn consume(&mut self, event: WindowEvent) {
		let (key, action, modifiers) = match event {
			WindowEvent::MouseButton(button, action, modifiers) => {
				(ButtonKey::Mouse(button), action, modifiers)
			}
			WindowEvent::Key(key, _, action, modifiers) => {
				(ButtonKey::Keyboard(key), action, modifiers)
			}
			_ => {
				return;
			}
		};

		if let Some(subscribers) = self.button_lookup.get(&key) {
			for id in subscribers {
				self.button_subscribers[*id as usize].event(action, modifiers);
			}
		}
	}
}