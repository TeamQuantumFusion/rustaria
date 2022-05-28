use crate::settings::CHUNK_SIZE;
use crate::ty;
use crate::ty::Direction;
use crate::ty::{Error, Offset, CHUNK_SIZE_F};
use euclid::Vector2D;
use num::FromPrimitive;

// ======================================== POSITION ========================================
#[derive(
	Copy,
	Clone,
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Default,
	Hash,
	serde::Serialize,
	serde::Deserialize,
	frogelua::FromLua,
)]
pub struct ChunkPos {
	pub x: u32,
	pub y: u32,
}

impl Offset<Direction> for ChunkPos {
	fn wrapping_offset(mut self, displacement: Direction) -> Self {
		use Direction::*;
		match displacement {
			Left => self.x = self.x.wrapping_sub(1),
			Right => self.x = self.x.wrapping_add(1),
			Down => self.y = self.y.wrapping_sub(1),
			Up => self.y = self.y.wrapping_add(1),
		}
		self
	}
	fn checked_offset(mut self, displacement: Direction) -> Option<Self> {
		use Direction::*;
		match displacement {
			Left => self.x = self.x.checked_sub(1)?,
			Right => self.x = self.x.checked_add(1)?,
			Down => self.y = self.y.checked_sub(1)?,
			Up => self.y = self.y.checked_add(1)?,
		};
		Some(self)
	}
}

impl Offset<(i32, i32)> for ChunkPos {
	fn wrapping_offset(self, (dx, dy): (i32, i32)) -> Self {
		// NOTE(leocth): no joke. this is how `wrapping_add_signed` is implemented.
		// https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#1205-1207

		Self {
			x: self.x + dx as u32,
			y: self.y + dy as u32,
		}
	}
	fn checked_offset(self, (dx, dy): (i32, i32)) -> Option<Self> {
		let x = ty::checked_add_signed_u32(self.x, dx)?;
		let y = ty::checked_add_signed_u32(self.y, dy)?;
		Some(Self { x, y })
	}
}

impl<S> TryFrom<Vector2D<f32, S>> for ChunkPos {
	type Error = Error;

	fn try_from(value: Vector2D<f32, S>) -> Result<Self, Self::Error> {
		Ok(ChunkPos {
			x: u32::from_f32(value.x / CHUNK_SIZE_F).ok_or(Error::OutOfBounds)?,
			y: u32::from_f32(value.y / CHUNK_SIZE_F).ok_or(Error::OutOfBounds)?,
		})
	}
}

impl TryFrom<(i64, i64)> for ChunkPos {
	type Error = Error;

	fn try_from(value: (i64, i64)) -> Result<Self, Self::Error> {
		Ok(ChunkPos {
			x: u32::from_i64(value.0 / CHUNK_SIZE as i64).ok_or(Error::OutOfBounds)?,
			y: u32::from_i64(value.1 / CHUNK_SIZE as i64).ok_or(Error::OutOfBounds)?,
		})
	}
}
