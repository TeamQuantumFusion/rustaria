use std::{
	fmt::{Debug, Formatter},
	ops::{Index, IndexMut},
};

use apollo::macros::*;
use euclid::{vec2, Vector2D};
use Direction::*;

// ======================================== DIRECTION ========================================
#[derive(
	Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Direction {
	Up,
	Left,
	Down,
	Right,
}

#[lua_impl(docs)]
impl Direction {
	pub fn vertical(self) -> bool {
		match self {
			Up | Down => true,
			Left | Right => false,
		}
	}

	pub fn horizontal(self) -> bool {
		match self {
			Up | Down => false,
			Left | Right => true,
		}
	}

	pub fn offset(self) -> (i8, i8) { (self.offset_x(), self.offset_y()) }
	pub fn offset_x(self) -> i8 {
		match self {
			Left => -1,
			Right => 1,
			Up | Down => 0,
		}
	}
	pub fn offset_y(self) -> i8 {
		match self {
			Up => 1,
			Down => -1,
			Left | Right => 0,
		}
	}

	pub fn clockwise(self) -> Self {
		match self {
			Up => Left,
			Left => Down,
			Down => Right,
			Right => Up,
		}
	}
	pub fn counterclockwise(self) -> Self {
		match self {
			Up => Right,
			Left => Up,
			Down => Left,
			Right => Down,
		}
	}
	pub fn rotate_180(self) -> Self {
		match self {
			Up => Down,
			Down => Up,
			Left => Right,
			Right => Left,
		}
	}
	pub fn horizontal_flip(self) -> Self {
		match self {
			Left => Right,
			Right => Left,
			other => other,
		}
	}
	pub fn vertical_flip(self) -> Self {
		match self {
			Up => Down,
			Down => Up,
			other => other,
		}
	}

	pub fn to_vec2<U>(self) -> Vector2D<f32, U> {
		vec2(self.offset_x() as f32, self.offset_y() as f32)
	}

	pub fn values() -> [Direction; 4] { [Up, Left, Down, Right] }
}

pub struct DirMap<V>([V; 4]);

impl<V> DirMap<V> {
	pub const fn new(value: [V; 4]) -> DirMap<V> { DirMap(value) }

	pub fn get_inner(self) -> [V; 4] { self.0 }

	pub fn get_ref_inner(&self) -> &[V; 4] { &self.0 }

	pub fn map<O: Default>(&self, mut func: impl FnMut(Direction, &V) -> O) -> DirMap<O> {
		let mut out = DirMap([O::default(), O::default(), O::default(), O::default()]);
		for dir in Direction::values() {
			out[dir] = func(dir, &self[dir]);
		}

		out
	}

	fn idx(dir: Direction) -> usize {
		match dir {
			Up => 0,
			Left => 1,
			Down => 2,
			Right => 3,
		}
	}
}

impl<D: Debug> Debug for DirMap<D> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl<D: Default> Default for DirMap<D> {
	fn default() -> Self { DirMap::new([D::default(), D::default(), D::default(), D::default()]) }
}

impl<V: Clone> Clone for DirMap<V> {
	fn clone(&self) -> Self { DirMap::new(self.0.clone()) }
}

impl<V: Copy> Copy for DirMap<V> {}

impl<V> Index<Direction> for DirMap<V> {
	type Output = V;

	fn index(&self, index: Direction) -> &Self::Output { &self.0[Self::idx(index)] }
}

impl<V> IndexMut<Direction> for DirMap<V> {
	fn index_mut(&mut self, index: Direction) -> &mut Self::Output { &mut self.0[Self::idx(index)] }
}
