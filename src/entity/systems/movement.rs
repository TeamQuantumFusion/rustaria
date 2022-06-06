use rsa_core::settings::UPS;

use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::humanoid::HumanoidComp;
use crate::entity::component::physics::PhysicsComp;
use crate::entity::component::pos::PositionComp;
use crate::entity::EntityStorage;

#[derive(Default)]

pub(crate) struct MovementECSystem;

impl MovementECSystem {
	pub(crate) fn tick(&self,  storage: &mut EntityStorage) {
		// Humanoids
		let query_mut = storage.query_mut::<(&mut HumanoidComp, &mut PhysicsComp, &mut PositionComp, &HitboxComp)>();
		for (_, (humanoid, physics, position, hitbox)) in query_mut {
			physics.velocity.x += humanoid.direction.x * (humanoid.settings.run_acceleration / UPS as f32);
			physics.velocity.y += humanoid.direction.y * (humanoid.settings.run_acceleration / UPS as f32);


			if physics.velocity.x > humanoid.settings.run_max_speed / UPS as f32 {
				physics.velocity.x = humanoid.settings.run_max_speed / UPS as f32;
			} else if physics.velocity.x < -(humanoid.settings.run_max_speed / UPS as f32) {
				physics.velocity.x = -humanoid.settings.run_max_speed / UPS as f32;
			}

			if hitbox.touches_ground {
				if physics.velocity.x > humanoid.settings.run_slowdown / UPS as f32 {
					physics.velocity.x -= humanoid.settings.run_slowdown / UPS as f32;
				} else if physics.velocity.x < -(humanoid.settings.run_slowdown / UPS as f32) {
					physics.velocity.x += humanoid.settings.run_slowdown / UPS as f32;
				} else {
					physics.velocity.x = 0.0;
				}

				if humanoid.jumping {
					humanoid.jump_frames_remaining = humanoid.settings.jump_frames;
				}
			}

			if humanoid.jump_frames_remaining > 0 {
				if humanoid.jumping {
					physics.velocity.y = humanoid.settings.jump_speed / UPS as f32;
					humanoid.jump_frames_remaining -= 1;
				} else {
					humanoid.jump_frames_remaining = 0;
				}
			}
		}
	}
}