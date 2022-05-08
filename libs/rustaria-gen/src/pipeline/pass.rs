use std::ops::Range;

#[derive(Clone, Eq, PartialEq)]
pub struct Pass {
	pub x_range: Range<u32>,
	pub y_range: Range<u32>,
}

impl Pass {
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
