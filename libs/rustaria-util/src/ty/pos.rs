use crate::ty::{TilePos, CHUNK_SIZE_F};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const ZERO: Pos = Pos { x: 0.0, y: 0.0 };
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
#[use_default]
pub struct Pos {
	pub x: f32,
	pub y: f32,
}

impl Pos {
	pub fn lerp(&self, other: &Pos, delta: f32) -> Pos {
		Pos {
			x: (self.x * (delta)) + (other.x * (1.0 - delta)),
			y: (self.y * (delta)) + (other.y * (1.0 - delta)),
		}
	}

	pub fn distance(&self, other: &Pos) -> f32 {
		let x = (other.x - self.x);
		let y = (other.y - self.y);
		((x * x) + (y * y)).sqrt()
	}
}

impl Add for Pos {
	type Output = Pos;

	fn add(self, rhs: Self) -> Self::Output {
		Pos {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}
impl AddAssign for Pos {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
impl Sub for Pos {
	type Output = Pos;

	fn sub(self, rhs: Self) -> Self::Output {
		Pos {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}
impl SubAssign for Pos {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl From<TilePos> for Pos {
	fn from(pos: TilePos) -> Self {
		Pos {
			x: (pos.chunk.x as f32 * CHUNK_SIZE_F) + pos.sub.x() as f32,
			y: (pos.chunk.y as f32 * CHUNK_SIZE_F) + pos.sub.y() as f32,
		}
	}
}

impl From<[f32; 2]> for Pos {
	fn from([x, y]: [f32; 2]) -> Self {
		Pos { x, y }
	}
}
impl From<Pos> for [f32; 2] {
	fn from(pos: Pos) -> Self {
		[pos.x, pos.y]
	}
}
