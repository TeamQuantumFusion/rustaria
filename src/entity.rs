use crate::{
    comps::{health::Health, physics::Physics},
    world::World,
};
use uuid::Uuid;

entity_proto_impl!(
    health => Health,
    physics => Physics
);

macro_rules! entity_proto_impl {
    ($($field:ident => $ty:ty),+) => {
        pub struct EntityPrototype {
            $(
                $field: Option<$ty>,
            )+
        }

        impl EntityPrototype {
            fn spawn(&self, world: &mut World) {
                let id = Uuid::new_v4();
                $(
                    if let Some($field) = self.$field.clone() {
                        world.comps.$field.insert(id, $field);
                    }
                )+
            }
        }
    };
}
use entity_proto_impl;
