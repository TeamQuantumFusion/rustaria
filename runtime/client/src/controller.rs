use glfw::Action;
use glfw::Modifiers;

use rustaria_controller::button::ButtonSubscriber;
use rustaria_util::info;

pub struct PrintSubscriber {
    pub message: &'static str,
}

impl ButtonSubscriber for PrintSubscriber {
    fn event(&mut self, action: Action, modifiers: Modifiers) {
        info!("{} {:?} {:?}", self.message, action, modifiers);
    }
}
