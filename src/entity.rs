use crate::{
    comps::{health::HealthPrototype, physics::PhysicsPrototype, ToComponent},
    world::World,
};
use mlua::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

entity_proto_impl!(
    health => HealthPrototype,
    physics => PhysicsPrototype,
);

macro_rules! entity_proto_impl {
    ($($field:ident => $ty:ty),+ $(,)?) => {
        #[derive(Debug, Clone, Deserialize)]
        pub struct EntityPrototype {
            $(
                $field: Option<$ty>,
            )+
        }

        impl EntityPrototype {
            pub fn spawn(&self, world: &mut World) {
                let id = Uuid::new_v4();
                $(
                    if let Some($field) = &self.$field {
                        world.comps.$field.insert(id, $field.to_component());
                    }
                )+
            }
        }
        impl LuaUserData for EntityPrototype {}
    };
}
use entity_proto_impl;
