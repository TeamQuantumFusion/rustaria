//! Systems and stuff

use crate::{
	debug::{DebugCategory, DebugRendererImpl},
	draw_debug,
	world::entity::{
		component::{GravityComponent, PhysicsComponent, PositionComponent},
		EntityStorage,
	},
	TPS,
};

pub mod collision;
pub mod humanoid;
pub mod network;

pub struct VelocitySystem;

impl VelocitySystem {
	pub fn tick(&mut self, world: &mut EntityStorage, debug: &mut impl DebugRendererImpl) {
		for (_, (position, velocity)) in
			world.query_mut::<(&mut PositionComponent, &mut PhysicsComponent)>()
		{
			draw_debug!(
				debug,
				DebugCategory::EntityVelocity,
				(
					position.pos,
					position.pos + (velocity.vel * (TPS as f32 / 30.0))
				),
				0xff6188,
				1.0
			);
			position.pos += velocity.vel;
			velocity.vel += velocity.accel;
		}
	}
}

pub struct GravitySystem;

impl GravitySystem {
	pub fn tick(&mut self, world: &mut EntityStorage) {
		for (_, (velocity, gravity)) in
			world.query_mut::<(&mut PhysicsComponent, &GravityComponent)>()
		{
			velocity.vel.y -= (0.8) / TPS as f32;
			// terminal velocity
			velocity.vel.y = velocity.vel.y.max(-(37.5 / TPS as f32));
		}
	}
}
