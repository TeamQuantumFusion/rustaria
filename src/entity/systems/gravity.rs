use rsa_core::settings::UPS;
use crate::entity::component::gravity::GravityComp;
use crate::entity::component::physics::PhysicsComp;
use crate::entity::EntityStorage;

pub(crate) struct GravityECSystem {
	gravity_pull: f32
}

impl GravityECSystem {
	pub(crate) fn new() -> Self {
		Self {
			gravity_pull: 20.0
		}
	}
	pub(crate) fn tick(&self, storage: &mut EntityStorage, delta: f32) {
		for (_, (gravity, physics)) in storage.query_mut::<(&GravityComp, &mut PhysicsComp)>() {
			self.tick_entity(gravity, physics, delta);
		}
	}

	#[inline(always)]
	pub(crate) fn tick_entity(&self, gravity: &GravityComp, physics: &mut PhysicsComp, delta: f32) {
		physics.velocity.y -= ((self.gravity_pull * gravity.speed) / UPS as f32) * delta;
	}
}