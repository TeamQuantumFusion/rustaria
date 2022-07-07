use rsa_core::TPS;
use rsa_core::ty::Direction;
use crate::entity::component::{CollisionComponent, HumanoidComponent, PhysicsComponent};
use crate::entity::EntityStorage;

pub struct HumanoidSystem;

impl HumanoidSystem {
	pub fn tick(&mut self, storage: &mut EntityStorage) {
		for (_, (physics, humanoid, collision)) in storage.query_mut::<(
			&mut PhysicsComponent,
			&mut HumanoidComponent,
			&CollisionComponent,
		)>() {
			//physics.vel.x += humanoid.dir.x * (humanoid.run_acceleration / TPS as f32);
			//physics.vel.y += humanoid.dir.y * (humanoid.run_acceleration / TPS as f32);

			//if physics.vel.x > humanoid.run_max_speed / TPS as f32 {
			//	physics.vel.x = humanoid.run_max_speed / TPS as f32;
			//} else if physics.vel.x < -(humanoid.run_max_speed / TPS as f32) {
			//	physics.vel.x = -humanoid.run_omax_speed / TPS as f32;
			//}

			if humanoid.dir.x < 0.0 && physics.vel.x > -(humanoid.run_max_speed / TPS as f32) {
				if physics.vel.x > humanoid.run_slowdown / TPS as f32 {
					physics.vel.x -= humanoid.run_slowdown / TPS as f32;
				}
				physics.vel.x -= humanoid.run_acceleration / TPS as f32;
			} else if humanoid.dir.x > 0.0 && physics.vel.x < humanoid.run_max_speed / TPS as f32 {
				if physics.vel.x < -(humanoid.run_slowdown / TPS as f32) {
					physics.vel.x += humanoid.run_slowdown / TPS as f32;
				}
				physics.vel.x += humanoid.run_acceleration / TPS as f32;
			} else if collision.collided[Direction::Up] {
				if physics.vel.x > humanoid.run_slowdown / TPS as f32 {
					physics.vel.x -= humanoid.run_slowdown / TPS as f32;
				} else if physics.vel.x < -(humanoid.run_slowdown / TPS as f32) {
					physics.vel.x += humanoid.run_slowdown / TPS as f32;
				} else {
					physics.vel.x = 0.0;
				}
			}

			if humanoid.jumping {
				if humanoid.jump_ticks_remaining > 0 {
					// If you still hold jump while jumping you go higher
					physics.vel.y = physics.vel.y.max(humanoid.jump_speed / TPS as f32);
					humanoid.jump_ticks_remaining -= 1;
				} else if collision.collided[Direction::Up] {
					// Start the jump
					physics.vel.y = physics.vel.y.max(humanoid.jump_speed / TPS as f32);
					humanoid.jump_ticks_remaining = (humanoid.jump_amount * TPS as f32) as u32;
				}
			} else {
				humanoid.jump_ticks_remaining = 0;
			}
		}
	}
}

//
