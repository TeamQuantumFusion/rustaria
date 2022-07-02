use anyways::Result;
use apollo::{LuaScope, LuaSerdeExt, Value};
use euclid::{rect, vec2, Rect, Size2D, Vector2D};

use crate::{
	debug::{DebugCategory, DebugRendererImpl},
	draw_debug,
	ty::{block_pos::BlockPos, direction::DirMap, WS},
	util::aabb,
	world::{
		chunk::CHUNK_SIZE,
		entity::{
			component::{CollisionComponent, PhysicsComponent, PositionComponent},
			EntityStorage,
		},
	},
	Api, ChunkStorage,
};

pub struct CollisionSystem;

impl CollisionSystem {
	pub fn tick(
		&mut self,
		api: &Api,
		storage: &mut EntityStorage,
		chunks: &mut ChunkStorage,
		debug: &mut impl DebugRendererImpl,
	) -> Result<()> {
		for (_, (collision, position, physics)) in storage.query_mut::<(
			&mut CollisionComponent,
			&PositionComponent,
			&mut PhysicsComponent,
		)>() {
			collision.collided = DirMap::new([false; 4]);

			// hitbox is the hitbox so we need to offset it to WorldSpace.
			let mut old_rect = collision.collision_box;
			old_rect.origin += position.pos;

			let mut new_rect = old_rect;
			new_rect.origin += physics.vel;

			let x1 = new_rect.min_x().min(old_rect.min_x()).floor() as i64;
			let y1 = new_rect.min_y().min(old_rect.min_y()).floor() as i64;
			let x2 = new_rect.max_x().max(old_rect.max_x()).ceil() as i64;
			let y2 = new_rect.max_y().max(old_rect.max_y()).ceil() as i64;
			draw_debug!(
				debug,
				DebugCategory::EntityCollision,
				rect(
					x1 as f32,
					y1 as f32,
					x2 as f32 - x1 as f32,
					y2 as f32 - y1 as f32,
				)
			);
			draw_debug!(debug, DebugCategory::EntityCollision, old_rect, 0xfcfcfa);

			collision.collisions.clear();
			for x in x1..x2 {
				for y in y1..y2 {
					let pos = vec2(x as f32, y as f32);
					if let Ok(world_pos) = BlockPos::try_from(pos) {
						if let Some(chunk) = chunks.get(world_pos.chunk) {
							for (id, layer) in chunk.layers.iter() {
								let prototype = api.carrier.block_layer.get(id);
								if !prototype.collision || !layer[world_pos.entry].collision {
									// dont move.
									continue;
								}
								let tile = Rect::new(pos.to_point(), Size2D::new(1.0, 1.0));
								test_collision(
									physics.vel,
									old_rect,
									tile,
									&mut collision.collisions,
									debug,
								);
							}
						}
					}
				}
			}

			// world border
			let w = chunks.width() as f32 * CHUNK_SIZE as f32;
			let h = chunks.height() as f32 * CHUNK_SIZE as f32;
			test_collision(
				physics.vel,
				old_rect,
				rect(0.0, -2.0, w, 2.0),
				&mut collision.collisions,
				debug,
			);
			test_collision(
				physics.vel,
				old_rect,
				rect(-2.0, 0.0, 2.0, h),
				&mut collision.collisions,
				debug,
			);
			test_collision(
				physics.vel,
				old_rect,
				rect(w, 0.0, 2.0, h),
				&mut collision.collisions,
				debug,
			);
			test_collision(
				physics.vel,
				old_rect,
				rect(0.0, h, w, 2.0),
				&mut collision.collisions,
				debug,
			);

			collision.collisions.sort_by(|v0, v1| v0.1.total_cmp(&v1.1));

			for (pos, _) in &mut collision.collisions {
				if let Some(Some((d, contact))) =
					aabb::resolve_dynamic_rect_vs_rect(physics.vel, old_rect, 1.0, *pos)
				{
					draw_debug!(
						debug,
						DebugCategory::EntityCollision,
						*pos,
						0xff0000,
						1.0,
						1.0
					);
					physics.vel += d;
					physics.accel += contact
						.to_vec2()
						.component_mul(vec2(physics.accel.x.abs(), physics.accel.y.abs()));
					collision.collided[contact] = true;
					if let Some(callback) = &collision.hit_callback {
						let chunks_scope = LuaScope::from(&mut *chunks);
						let contact = api.luna.lua.to_value(&contact).unwrap();
						let _result: Value = callback.call((chunks_scope.lua(), contact))?;
					}
				}
			}
		}
		Ok(())
	}
}

fn test_collision(
	vel: Vector2D<f32, WS>,
	collision_area: Rect<f32, WS>,
	rect: Rect<f32, WS>,
	collisions: &mut Vec<(Rect<f32, WS>, f32)>,
	debug: &mut impl DebugRendererImpl,
) {
	if let Some((pos, contact_time)) = aabb::dynamic_rect_vs_rect(vel, collision_area, 1.0, rect)
		.map(|collision| (rect, collision.contact_time))
	{
		draw_debug!(debug, DebugCategory::EntityCollision, rect, 0x939293);
		collisions.push((pos, contact_time));
	}
}
