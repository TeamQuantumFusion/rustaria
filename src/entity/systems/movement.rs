use std::collections::hash_map::Entry;
use std::collections::HashMap;
use hecs::Entity;
use rsa_core::logging::trace;
use rsa_core::math::default::Vector2D;
use rsa_core::settings::UPS;

use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::humanoid::HumanoidComp;
use crate::entity::component::physics::PhysicsComp;
use crate::entity::EntityStorage;

#[derive(Default)]
pub(crate) struct MovementECSystem {
	events: HashMap<Entity, Vec<Vector2D<f32>>>
}

impl MovementECSystem {
	pub(crate) fn tick(&mut self, storage: &mut EntityStorage, delta: f32) {
		// Humanoids
		for (entity, (humanoid, physics,hitbox)) in storage.query_mut::<(&mut HumanoidComp, &mut PhysicsComp, &HitboxComp)>() {
			self.tick_entity(entity, humanoid, physics, hitbox, delta);
		}
	}

	#[inline(always)]
	pub(crate) fn tick_entity(&mut self, entity: Entity, humanoid: &mut HumanoidComp, physics: &mut PhysicsComp, hitbox: &HitboxComp, delta: f32) {
		physics.velocity.x += (humanoid.dir.x * (humanoid.settings.run_acceleration / UPS as f32)) * delta;
		physics.velocity.y += (humanoid.dir.y * (humanoid.settings.run_acceleration / UPS as f32)) * delta;


		if physics.velocity.x > humanoid.settings.run_max_speed / UPS as f32 {
			physics.velocity.x = humanoid.settings.run_max_speed / UPS as f32;
		} else if physics.velocity.x < -(humanoid.settings.run_max_speed / UPS as f32) {
			physics.velocity.x = -humanoid.settings.run_max_speed / UPS as f32;
		}

		if hitbox.touches_ground {
			if physics.velocity.x > humanoid.settings.run_slowdown / UPS as f32 {
				physics.velocity.x -= (humanoid.settings.run_slowdown / UPS as f32) * delta;
			} else if physics.velocity.x < -(humanoid.settings.run_slowdown / UPS as f32) {
				physics.velocity.x += (humanoid.settings.run_slowdown / UPS as f32) * delta;
			} else {
				physics.velocity.x = 0.0;
			}

			if humanoid.jumping {
				humanoid.jump_frames_remaining = humanoid.settings.jump_frames as f32;
			}
		}

		if humanoid.jump_frames_remaining > 0.0 {
			if humanoid.jumping {
				physics.velocity.y = humanoid.settings.jump_speed / UPS as f32;
				humanoid.jump_frames_remaining -= delta;
			} else {
				humanoid.jump_frames_remaining = 0.0;
			}
		}
	}
}