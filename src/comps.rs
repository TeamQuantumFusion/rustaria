pub mod health;
pub mod physics;

use std::collections::HashMap;

use uuid::Uuid;

use self::{health::Health, physics::Physics};

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

type CompId = Uuid;
