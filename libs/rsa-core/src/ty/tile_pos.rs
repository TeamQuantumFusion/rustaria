use std::fmt::{Display, Formatter};

use euclid::Vector2D;

use crate::settings::CHUNK_SIZE;
use crate::ty::chunk_pos::ChunkPos;
use crate::ty::chunk_sub_pos::ChunkSubPos;
use crate::ty::Error::OutOfBounds;
use crate::ty::{Error, Offset};

#[derive(
	Copy,
	Clone,
	PartialOrd,
	PartialEq,
	Debug,
	Default,
	serde::Serialize,
	serde::Deserialize,
	frogelua::FromLua,
)]
pub struct TilePos {
	pub chunk: ChunkPos,
	pub sub: ChunkSubPos,
}

impl TilePos {
	pub fn x(&self) -> i64 {
		(self.chunk.x as i64 * CHUNK_SIZE as i64) + self.sub.x() as i64
	}

	pub fn y(&self) -> i64 {
		(self.chunk.y as i64 * CHUNK_SIZE as i64) + self.sub.y() as i64
	}
}

impl Offset<(i8, i8)> for TilePos {
	fn wrapping_offset(self, displacement @ (dx, dy): (i8, i8)) -> Self {
		match Self::checked_offset(self, displacement) {
			Some(s) => s,
			None => Self {
				chunk: self.chunk.wrapping_offset((dx as i32, dy as i32)),
				sub: self.sub.euclid_offset(displacement),
			},
		}
	}

	fn checked_offset(self, displacement @ (dx, dy): (i8, i8)) -> Option<Self> {
		Some(match self.sub.checked_offset(displacement) {
			Some(sub) => Self {
				chunk: self.chunk,
				sub,
			},
			None => Self {
				chunk: self.chunk.checked_offset((dx as i32, dy as i32))?,
				sub: self.sub.euclid_offset(displacement),
			},
		})
	}
}

impl<S> TryFrom<Vector2D<f32, S>> for TilePos {
	type Error = Error;

	fn try_from(value: Vector2D<f32, S>) -> Result<Self, Self::Error> {
		Ok(TilePos {
			chunk: ChunkPos::try_from(value)?,
			sub: ChunkSubPos::try_new(
				(value.x as i64 % CHUNK_SIZE as i64) as u8,
				(value.y as i64 % CHUNK_SIZE as i64) as u8,
			)
			.ok_or(OutOfBounds)?,
		})
	}
}

impl Display for TilePos {
	//123, 432 (3:0@4:4)
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let x = (self.chunk.x as i64 * CHUNK_SIZE as i64) + self.sub.x() as i64;
		let y = (self.chunk.y as i64 * CHUNK_SIZE as i64) + self.sub.y() as i64;
		f.write_str(&format!(
			"{x}, {y} ({}:{}@{}:{})",
			self.chunk.x,
			self.chunk.y,
			self.sub.x(),
			self.sub.y()
		))
	}
}
