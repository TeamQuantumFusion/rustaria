use rsa_core::settings::UPS;

use crate::entity::component::physics::PhysicsComp;
use crate::entity::component::pos::PositionComp;
use crate::entity::EntityStorage;

#[derive(Default)]
pub(crate) struct PhysicsECSystem;

impl PhysicsECSystem {
	pub(crate) fn tick(&self, storage: &mut EntityStorage, delta: f32) {
		for (_, (position, physics)) in storage.query_mut::<(&mut PositionComp, &mut PhysicsComp)>()
		{
			self.tick_entity(position, physics, delta)
		}
	}

	#[inline(always)]
	pub(crate) fn tick_entity(
		&self,
		position: &mut PositionComp,
		physics: &mut PhysicsComp,
		delta: f32,
	) {
		position.position += (physics.velocity / UPS as f32) * delta;
		physics.velocity += (physics.acceleration / UPS as f32) * delta;
	}
}
