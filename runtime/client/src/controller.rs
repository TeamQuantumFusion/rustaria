use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};
use rustaria_controller::button::ButtonSubscriber;

use rustaria_util::info;
use crate::warn;

pub struct PrintSubscriber {
    pub message: &'static str
}

impl ButtonSubscriber for PrintSubscriber {
    fn event(&mut self,  action: Action, modifiers: Modifiers) {
        info!("{} {:?} {:?}", self.message, action, modifiers);
    }
}
