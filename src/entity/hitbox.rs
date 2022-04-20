use crate::chunk::{Chunk, ChunkContainer};
use crate::ChunkManager;
use rustaria_util::info;
use rustaria_util::ty::pos::{Pos, ZERO};
use rustaria_util::ty::{ChunkPos, ChunkSubPos, Offset, Rectangle, TilePos, CHUNK_SIZE};
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct HitboxComp {
	pub hitbox: Rectangle,
}

impl HitboxComp {
	pub fn calc(
		&self,
		rect: Rectangle,
		start_pos: Pos,
		velocity: Pos,
		chunks: &ChunkContainer,
	) -> Option<(Pos, bool)> {
		let caster = RayCaster::new(rect, start_pos, velocity, chunks)?;
		caster.hit()

		//let next_pos = pos + velocity;
		//// most cave man brain collision ever. Go below to the raycaster to experience undone DDA over-engineering.
		//let next_pos_tile = TilePos::try_from(next_pos).ok()?;
		//let hit = chunks
		//	.get_chunk(next_pos_tile.chunk)?
		//	.tiles
		//	.index(next_pos_tile.sub)
		//	.collision;
		//Some(if hit { pos } else { next_pos })
	}
}

pub struct RayCaster<'a> {
	rect: Rectangle,
	pos: Pos,
	velocity: Pos,
	chunks: &'a ChunkContainer,

	// cache
	last_chunk_pos: ChunkPos,
	last_chunk: &'a Chunk,
}

// EVERYTHING IS BASED IN THE BOTTOM LEFT
impl<'a> RayCaster<'a> {
	fn new(rect: Rectangle, pos: Pos, velocity: Pos, chunks: &ChunkContainer) -> Option<RayCaster> {
		let chunk_pos = ChunkPos::try_from(pos).ok()?;
		Some(RayCaster {
			rect,
			pos,
			velocity,
			last_chunk: chunks.get_chunk(chunk_pos)?,
			last_chunk_pos: chunk_pos,
			chunks,
		})
	}

	// if its hits something it cancels instantly
	pub fn hit(mut self) -> Option<(Pos, bool)> {
		if self.initial_check()? {
			return Some((self.pos, true));
		}

		let top = self.velocity.x.max(self.velocity.y);
		let ray_dir_x = self.velocity.x / top;
		let ray_dir_y = self.velocity.y / top;

		let mut map_pos = TilePos::try_from(self.pos).ok()?;

		let delta_dist_x = calc_delta(ray_dir_x);
		#[rustfmt::skip]
		let (offset_x, step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
			(self.rect.x, -1, (self.pos.x - (self.pos.x as i64 as f32)) * delta_dist_x)
		} else {
			(self.rect.x + self.rect.width, 1, ((self.pos.x as i64 as f32) + 1.0 - self.pos.x) * delta_dist_x)
		};

		let delta_dist_y = calc_delta(ray_dir_y);
		#[rustfmt::skip]
		let (offset_y, step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
			(self.rect.y, -1, (self.pos.y - (self.pos.y as i64 as f32)) * delta_dist_y)
		} else {
			(self.rect.y + self.rect.height, 1, ((self.pos.y as i64 as f32) + 1.0 - self.pos.y) * delta_dist_y)
		};

		let distance = ZERO.distance(&self.velocity);
		let mut current_distance = 0.0;

		let rect_x = self.rect.x as i64;
		let rect_y = self.rect.y as i64;
		let rect_x2 = (self.rect.x + self.rect.width) as i64;
		let rect_y2 = (self.rect.y + self.rect.height) as i64;
		while current_distance < distance {
			if side_dist_x < side_dist_y {
				side_dist_x += delta_dist_x;
				map_pos = map_pos.checked_offset((step_x, 0))?;
				current_distance = side_dist_x;

				let y = rect_y + map_pos.y();
				let y_height = rect_y2 + map_pos.y();
				let x = offset_x as i64 + map_pos.x();
				for y in y..=y_height {
					if self.check(TilePos {
						chunk: ChunkPos::try_from((x, y)).ok()?,
						sub: ChunkSubPos::new(
							(x % CHUNK_SIZE as i64) as u8,
							(y % CHUNK_SIZE as i64) as u8,
						),
					}) {
						return Some((Pos::from(map_pos), true));
					}
				}
			} else {
				side_dist_y += delta_dist_y;
				map_pos = map_pos.checked_offset((0, step_y))?;
				current_distance = side_dist_y;

				let x = rect_x + map_pos.x();
				let x_height = rect_x2 + map_pos.x();
				let y = offset_y as i64 + map_pos.y();
				for x in x..=x_height {
					if self.check(TilePos {
						chunk: ChunkPos::try_from((x, y)).ok()?,
						sub: ChunkSubPos::new(
							(x % CHUNK_SIZE as i64) as u8,
							(y % CHUNK_SIZE as i64) as u8,
						),
					}) {
						return Some((Pos::from(map_pos), true));
					}
				}
			}

			if self.check(map_pos) {
				return Some((Pos::from(map_pos), true));
			}
		}

		Some((self.pos + self.velocity, false))
	}

	// checks if the rect is already hitting something.
	fn initial_check(&mut self) -> Option<bool> {
		let x = (self.rect.x + self.pos.x) as i64;
		let y = (self.rect.y + self.pos.y) as i64;
		let x_width = (self.rect.x + self.rect.width + self.pos.x) as i64;
		let y_height = (self.rect.y + self.rect.height + self.pos.y) as i64;
		for y in y..=y_height {
			for x in x..=x_width {
				if self.check(TilePos {
					chunk: ChunkPos::try_from((x, y)).ok()?,
					sub: ChunkSubPos::new(
						(x % CHUNK_SIZE as i64) as u8,
						(y % CHUNK_SIZE as i64) as u8,
					),
				}) {
					return Some(true);
				}
			}
		}

		Some(false)
	}

	fn check(&mut self, pos: TilePos) -> bool {
		if self.last_chunk_pos != pos.chunk {
			if let Some(chunk) = self.chunks.get_chunk(pos.chunk) {
				self.last_chunk = chunk;
				self.last_chunk_pos = pos.chunk;
			} else {
				return true;
			}
		}

		self.last_chunk.tiles.index(pos.sub).collision
	}
}

fn calc_delta(ray_dir: f32) -> f32 {
	if ray_dir == 0.0 {
		1.0
	} else {
		(1.0 / ray_dir).abs()
	}
}
