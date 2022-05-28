use crate::settings::CHUNK_SIZE;
use crate::ty;
use crate::ty::{Offset, CHUNK_SIZE_MASK, CHUNK_SIZE_U8};
use mlua::{Error, FromLua, Lua, Value};

#[derive(
	Copy, Clone, PartialOrd, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkSubPos(u8);

impl ChunkSubPos {
	pub fn new(x: u8, y: u8) -> Self {
		assert!(x < CHUNK_SIZE_U8, "x is out-of-bounds: {x} >= {CHUNK_SIZE}");
		assert!(y < CHUNK_SIZE_U8, "y is out-of-bounds: {y} >= {CHUNK_SIZE}");
		Self::new_unchecked(x, y)
	}
	pub fn try_new(x: u8, y: u8) -> Option<Self> {
		if x >= CHUNK_SIZE_U8 || y >= CHUNK_SIZE_U8 {
			None
		} else {
			Some(Self::new_unchecked(x, y))
		}
	}
	fn new_unchecked(x: u8, y: u8) -> Self {
		Self((x << 4) | y)
	}

	pub fn x(self) -> u8 {
		self.0 >> 4
	}
	pub fn y(self) -> u8 {
		self.0 & CHUNK_SIZE_MASK
	}
	pub fn euclid_offset(self, (dx, dy): (i8, i8)) -> Self {
		// SAFETY: `rem_euclid` returns a number lesser than `CHUNK_SIZE`.
		Self::new_unchecked(
			(self.x() as i16 + dx as i16).rem_euclid(CHUNK_SIZE as i16) as u8,
			(self.y() as i16 + dy as i16).rem_euclid(CHUNK_SIZE as i16) as u8,
		)
	}
}

// TODO(leocth): add Offset<ChunkSubPos> impl for (i8, i8)
impl Offset<(i8, i8)> for ChunkSubPos {
	fn wrapping_offset(self, (dx, dy): (i8, i8)) -> Self {
		// NOTE(leocth): no joke. this is how `wrapping_add_signed` is implemented.
		// https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#1205-1207

		let x = (self.x() + dx as u8) & CHUNK_SIZE_MASK;
		let y = (self.y() + dy as u8) & CHUNK_SIZE_MASK;
		// SAFETY: x and y are no greater than or equal to CHUNK_SIZE after ANDing with CHUNK_SIZE_MASK.
		Self::new_unchecked(x, y)
	}

	fn checked_offset(self, (dx, dy): (i8, i8)) -> Option<Self> {
		let x = ty::checked_add_signed_u8(self.x(), dx)?;
		let y = ty::checked_add_signed_u8(self.y(), dy)?;
		Self::try_new(x, y)
	}
}

impl FromLua for ChunkSubPos {
	fn from_lua(lua_value: Value, _: &Lua) -> mlua::Result<Self> {
		if let Value::Table(table) = lua_value {
			let x = table.get("x")?;
			let y = table.get("y")?;
			Ok(ChunkSubPos::new(x, y))
		} else {
			Err(Error::UserDataTypeMismatch)
		}
	}
}
