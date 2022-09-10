use std::fmt::{Display, Formatter};

use apollo::{FromLua, Lua, ToLua, Value};
use rsa_core::{
	api::util::lua_table,
	debug::DebugDraw,
	err::Result,
	math::{rect, Vector2D},
	num::ToPrimitive,
	ty::{Error, Error::OutOfBounds, Offset},
};

use crate::{
	ty::{block_layer_pos::BlockLayerPos, chunk_pos::ChunkPos},
	CHUNK_SIZE,
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

	pub fn x(&self) -> u64 { (self.chunk.x as u64 * CHUNK_SIZE as u64) + self.entry.x() as u64 }

	pub fn y(&self) -> u64 { (self.chunk.y as u64 * CHUNK_SIZE as u64) + self.entry.y() as u64 }
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

impl<X: ToPrimitive, Y: ToPrimitive> TryFrom<(X, Y)> for BlockPos {
	type Error = Error;

	fn try_from((x, y): (X, Y)) -> Result<Self, Self::Error> {
		let x = x.to_u64().ok_or(OutOfBounds)?;
		let y = y.to_u64().ok_or(OutOfBounds)?;
		Ok(BlockPos {
			chunk: ChunkPos {
				x: u32::try_from(x / CHUNK_SIZE as u64)
					.ok()
					.ok_or(OutOfBounds)?,
				y: u32::try_from(y / CHUNK_SIZE as u64)
					.ok()
					.ok_or(OutOfBounds)?,
			},
			entry: BlockLayerPos::try_new(
				(x as u64 % CHUNK_SIZE as u64) as u8,
				(y as u64 % CHUNK_SIZE as u64) as u8,
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
			"x{x}, y{y} (cx{}:cy{}@ex{}:ey{})",
			self.chunk.x,
			self.chunk.y,
			self.entry.x(),
			self.entry.y()
		))
	}
}

impl ToLua for BlockPos {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(
			lua.create_table_from([("x", self.x()), ("y", self.y())])?,
		))
	}
}

impl FromLua for BlockPos {
	fn from_lua(lua_value: Value, _: &Lua) -> Result<Self> {
		let table = lua_table(lua_value)?;

		if table.contains_key("x")? && table.contains_key("y")? {
			Ok(BlockPos::try_from((
				table.get::<_, u64>("x")?,
				table.get::<_, u64>("y")?,
			))?)
		} else {
			Ok(BlockPos {
				chunk: table.get("chunk")?,
				entry: table.get("entry")?,
			})
		}
	}
}

impl Into<DebugDraw> for BlockPos {
	fn into(self) -> DebugDraw { DebugDraw::Quad(rect(self.x() as f32, self.y() as f32, 1.0, 1.0)) }
}
