use crate::{Climate, Generator, Zone};
use std::ops::Range;
use rustaria_common::ty::Direction;

pub mod sampler;

pub struct Sweep<'a, T: Clone + Default> {
	pub generator: &'a Generator<T>,
	pub x_range: Range<u32>,
	pub y_range: Range<u32>,
}
impl<'a, T: Clone + Default> Sweep<'a, T> {
	pub fn zone(gen: &'a Generator<T>, zone: &Zone) -> Sweep<'a, T> {
		Sweep {
			generator: gen,
			x_range: 0..gen.width,
			y_range: zone.world_range.clone(),
		}
	}

	pub fn climate(gen: &'a Generator<T>, zone: &Zone, climate: &Climate, climate_x_range: &Range<u32>) -> Sweep<'a, T> {
		let y_offset = zone.world_range.start;
		let height = ((zone.world_range.end - y_offset) as f32 * climate.depth) as u32;
		Sweep {
			generator: gen,
			x_range: climate_x_range.clone(),
			y_range: y_offset..(y_offset + height),
		}
	}

	pub fn apply(self, mut func: impl FnMut(&Self, u32, u32)) {
		for y in self.y_range.clone() {
			for x in self.x_range.clone() {
				func(&self, x, y);
			}
		}
	}

	// Extends this sweep by creating a new sweep with a new inner area to scan.
	pub fn extend(&self, width: Range<f32>, height: Range<f32>) -> Sweep<T> {
		Sweep {
			generator: self.generator,
			x_range: (self.min_x() + (self.width() as f32 * width.start) as u32)
				..(self.min_x() + (self.width() as f32 * width.end) as u32),
			y_range: (self.min_y() + (self.height() as f32 * height.start) as u32)
				..(self.min_y() + (self.height() as f32 * height.end) as u32),
		}
	}

	pub fn local_dir(&self, x: u32, y: u32, direction: Direction) -> f32 {
		match direction {
			Direction::Up => self.local_y(y),
			Direction::Right => self.local_x(x),
			Direction::Down => 1.0 - self.local_y(y),
			Direction::Left => 1.0 - self.local_x(x),
		}
	}

	#[inline(always)]
	pub fn local_x(&self, x: u32) -> f32 {
		(x - self.x_range.start) as f32 / self.width() as f32
	}

	#[inline(always)]
	pub fn local_y(&self, y: u32) -> f32 {
		(y - self.y_range.start) as f32 / self.height() as f32
	}

	#[inline(always)]
	pub fn max_x(&self) -> u32 {
		self.x_range.end
	}

	#[inline(always)]
	pub fn max_y(&self) -> u32 {
		self.y_range.end
	}

	#[inline(always)]
	pub fn min_x(&self) -> u32 {
		self.x_range.start
	}

	#[inline(always)]
	pub fn min_y(&self) -> u32 {
		self.y_range.start
	}

	#[inline(always)]
	pub fn width(&self) -> u32 {
		self.max_x() - self.min_x()
	}

	#[inline(always)]
	pub fn height(&self) -> u32 {
		self.max_y() - self.min_y()
	}
}
