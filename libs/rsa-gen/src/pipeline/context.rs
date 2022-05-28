use crate::Generator;
use rustaria_common::ty::Direction;
use std::ops::Range;

#[derive(Clone)]
pub struct Context<'a, T: Clone + Default + Send + Sync> {
	pub generator: &'a Generator<T>,
	pub x_range: Range<u32>,
	pub y_range: Range<u32>,
}
impl<'a, T: Clone + Default + Send + Sync> Context<'a, T> {
	// Extends this context by creating a new sampler with a new inner area to scan.
	pub fn extend(&self, width: Range<f32>, height: Range<f32>) -> Context<'a, T> {
		Context {
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
