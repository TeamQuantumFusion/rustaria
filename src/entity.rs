use crate::{
    comps::{health::HealthPrototype, physics::PhysicsPrototype, ToComponent},
    world::World,
};
use mlua::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct EntityPrototype {
    health: Option<HealthPrototype>,

    #[serde(default)]
    physics: PhysicsPrototype,
}

impl EntityPrototype {
    pub fn spawn(&self, world: &mut World) {
        let id = Uuid::new_v4();
        if let Some(pt) = &self.health {
            world.comps.health.insert(id, pt.to_component());
        }
        world.comps.physics.insert(id, self.physics.to_component());
    }
}
impl LuaUserData for EntityPrototype {}
