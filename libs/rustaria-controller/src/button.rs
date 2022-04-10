use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use glfw::{Action, Key, Modifiers, MouseButton};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ButtonKey {
    Keyboard(Key),
    Mouse(MouseButton),
}

pub trait ButtonSubscriber {
    fn event(&mut self, action: Action, modifiers: Modifiers);
}

// Builtin
#[derive(Default, Clone)]
pub struct HoldSubscriber(Arc<AtomicBool>);

impl HoldSubscriber {
    pub fn new() -> HoldSubscriber {
        HoldSubscriber(Arc::new(Default::default()))
    }

    pub fn held(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

impl ButtonSubscriber for HoldSubscriber {
    fn event(&mut self, action: Action, _: Modifiers) {
        self.0
            .store(!matches!(action, Action::Release), Ordering::Relaxed);
    }
}

#[derive(Default, Clone)]
pub struct TriggerSubscriber(Arc<AtomicU32>);

impl TriggerSubscriber {
    pub fn new() -> TriggerSubscriber {
        TriggerSubscriber(Arc::new(Default::default()))
    }

    pub fn triggered(&self) -> bool {
        let i = self.0.load(Ordering::Relaxed);
        if i > 0 {
            self.0.fetch_sub(1, Ordering::Relaxed);
            return true;
        }
        false
    }
}

impl ButtonSubscriber for TriggerSubscriber {
    fn event(&mut self, action: Action, _: Modifiers) {
        if action == Action::Release {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
}
