use rsa_core::settings::UPS;
use crate::entity::component::gravity::GravityComp;
use crate::entity::component::physics::PhysicsComp;
use crate::entity::EntityStorage;

#[derive(Default)]
pub(crate) struct GravityECSystem {
	gravity_pull: f32
}

impl GravityECSystem {
	pub(crate) fn tick(&self, storage: &mut EntityStorage) {
		let query_mut = storage.query_mut::<(&GravityComp, &mut PhysicsComp)>();
		for (_, (gravity, physics)) in query_mut {
			physics.velocity.y -= (self.gravity_pull * gravity.speed) / UPS as f32;
		}
	}
}