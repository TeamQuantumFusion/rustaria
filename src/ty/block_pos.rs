use std::fmt::{Display, Formatter};

use euclid::Vector2D;

use crate::{
	ty::{block_layer_pos::BlockLayerPos, chunk_pos::ChunkPos, Error, Error::OutOfBounds, Offset},
	world::chunk::CHUNK_SIZE,
};

#[derive(
	Copy,
	Clone,
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Hash,
	Debug,
	Default,
	serde::Serialize,
	serde::Deserialize,
)]
pub struct BlockPos {
	pub chunk: ChunkPos,
	pub entry: BlockLayerPos,
}

impl BlockPos {
	pub fn new(chunk: ChunkPos, entry: BlockLayerPos) -> BlockPos { BlockPos { chunk, entry } }

	pub fn x(&self) -> i64 { (self.chunk.x as i64 * CHUNK_SIZE as i64) + self.entry.x() as i64 }

	pub fn y(&self) -> i64 { (self.chunk.y as i64 * CHUNK_SIZE as i64) + self.entry.y() as i64 }
}

impl Offset<(i8, i8)> for BlockPos {
	fn wrapping_offset(self, displacement @ (dx, dy): (i8, i8)) -> Self {
		match Self::checked_offset(self, displacement) {
			Some(s) => s,
			None => Self {
				chunk: self.chunk.wrapping_offset((dx as i32, dy as i32)),
				entry: self.entry.euclid_offset(displacement),
			},
		}
	}

	fn checked_offset(self, displacement @ (dx, dy): (i8, i8)) -> Option<Self> {
		Some(match self.entry.checked_offset(displacement) {
			Some(sub) => Self {
				chunk: self.chunk,
				entry: sub,
			},
			None => Self {
				chunk: self.chunk.checked_offset((dx as i32, dy as i32))?,
				entry: self.entry.euclid_offset(displacement),
			},
		})
	}
}

impl<S> TryFrom<Vector2D<f32, S>> for BlockPos {
	type Error = Error;

	fn try_from(value: Vector2D<f32, S>) -> Result<Self, Self::Error> {
		Ok(BlockPos {
			chunk: ChunkPos::try_from(value)?,
			entry: BlockLayerPos::try_new(
				(value.x as i64 % CHUNK_SIZE as i64) as u8,
				(value.y as i64 % CHUNK_SIZE as i64) as u8,
			)
			.ok_or(OutOfBounds)?,
		})
	}
}

impl Display for BlockPos {
	//123, 432 (3:0@4:4)
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let x = (self.chunk.x as i64 * CHUNK_SIZE as i64) + self.entry.x() as i64;
		let y = (self.chunk.y as i64 * CHUNK_SIZE as i64) + self.entry.y() as i64;
		f.write_str(&format!(
			"{x}, {y} ({}:{}@{}:{})",
			self.chunk.x,
			self.chunk.y,
			self.entry.x(),
			self.entry.y()
		))
	}
}
