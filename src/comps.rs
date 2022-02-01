pub mod health;
pub mod physics;

use std::collections::HashMap;

use uuid::Uuid;

use self::{health::Health, physics::Physics};

#[derive(Debug, Clone)]
pub struct Comps {
    pub health: HashMap<CompId, Health>,
    pub physics: HashMap<CompId, Physics>,
}

impl Comps {
    pub fn new() -> Self {
        Self {
            health: HashMap::new(),
            physics: HashMap::new(),
        }
    }
    pub fn update(&mut self) {
        for (id, comp) in self.physics.iter_mut() {
            let health = self.health.get_mut(id);
            comp.update(health);
        }
    }
}

pub type CompId = Uuid;

pub trait ToComponent {
    type Comp;

    fn to_component(&self) -> Self::Comp;
}
