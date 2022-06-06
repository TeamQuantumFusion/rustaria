use std::ops::Index;

use hecs::World;

use rsa_core::math::{Rect, Size2D, vec2, Vector2D, WorldSpace};
use rsa_core::ty::TilePos;
use crate::chunk::ChunkSystem;

use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::physics::PhysicsComp;
use crate::entity::component::pos::PositionComp;
use crate::entity::EntityStorage;
use crate::util::aabb;

#[derive(Default)]
pub(crate) struct CollisionECSystem;

impl CollisionECSystem {
	pub(crate) fn tick(&self, storage: &mut EntityStorage, chunks: &ChunkSystem) {
		let query_mut = storage.query_mut::<(&PositionComp, &mut PhysicsComp, &mut HitboxComp)>();
		for (_, (position, physics, hitbox)) in query_mut {
			hitbox.touches_ground = false;

			// hitbox is the hitbox so we need to offset it to WorldSpace.
			let mut old_rect = hitbox.hitbox;
			old_rect.origin += position.position;

			let mut new_rect = old_rect;
			new_rect.origin += physics.velocity;

			let x1 = new_rect.min_x().min(old_rect.min_x()).floor() as i64;
			let y1 = new_rect.min_y().min(old_rect.min_y()).floor() as i64;
			let x2 = new_rect.max_x().max(old_rect.max_x()).ceil() as i64;
			let y2 = new_rect.max_y().max(old_rect.max_y()).ceil() as i64;

			let mut collisions = Vec::new();
			for x in x1..=x2 {
				for y in y1..=y2 {
					if let Some((pos, contact_time)) =
					test_tile(vec2(x as f32, y as f32), physics.velocity, old_rect, chunks)
					{
						collisions.push((pos, contact_time));
					}
				}
			}

			collisions.sort_by(|v0, v1| v0.1.total_cmp(&v1.1));

			for (pos, _) in collisions {
				if let Some((d, contact)) =
				aabb::resolve_dynamic_rect_vs_rect(physics.velocity, old_rect, 1.0, pos)
				{
					physics.velocity += d;
					physics.acceleration += contact.component_mul(vec2(
						physics.acceleration.x.abs(),
						physics.acceleration.y.abs(),
					));

					if contact == vec2(0.0, 1.0) {
						hitbox.touches_ground = true;
					}
				}
			}
		}
	}
}

fn test_tile(
	pos: Vector2D<f32, WorldSpace>,
	vel: Vector2D<f32, WorldSpace>,
	collision_area: Rect<f32, WorldSpace>,
	chunks: &ChunkSystem,
) -> Option<(Rect<f32, WorldSpace>, f32)> {
	const TILE_SIZE: Size2D<f32, WorldSpace> = Size2D::new(1.0, 1.0);

	let tile_pos = TilePos::try_from(pos).ok()?;
	let chunk = chunks.get_chunk(tile_pos.chunk)?;
	if !chunk.tiles.index(tile_pos.sub).collision {
		// dont move.
		return None;
	}
	let tile = Rect::new(pos.to_point(), TILE_SIZE);
	aabb::dynamic_rect_vs_rect(vel, collision_area, 1.0, tile)
		.map(|collision| (tile, collision.contact_time))
}
