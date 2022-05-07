use crate::{Climate, Generator, Noise, Zone};
use std::ops::Range;


pub struct WorldPass {
	pub world_width: u32,
	pub world_height: u32,
	pub width: Range<u32>,
	pub height: Range<u32>,
}

impl WorldPass {
	pub fn zone<T: Clone + Default>(gen: &Generator<T>, zone: &Zone) -> WorldPass {
		WorldPass {
			world_width: gen.width,
			world_height: gen.height,
			width: 0..gen.width,
			height: zone.world_range.clone(),
		}
	}

	pub fn climate<T: Clone + Default>(gen: &Generator<T>, zone: &Zone, climate_x_range: &Range<u32>, climate: &Climate) -> WorldPass {
		let x_offset = climate_x_range.start;
		let y_offset = zone.world_range.start;
		let width = climate_x_range.end - x_offset;
		let height = ((zone.world_range.end - y_offset) as f32 * climate.depth) as u32;
		let climate_x_range = climate_x_range.clone();
		let climate_y_range = y_offset..(y_offset + height);
		WorldPass {
			world_width: gen.width,
			world_height: gen.height,
			width: climate_x_range,
			height: climate_y_range,
		}
	}

	pub fn re_clamp(&self, width_clamp: Range<f32>, height_clamp: Range<f32>) -> WorldPass {
		let width = self.width();
		let height = self.height();
		WorldPass {
			world_width: self.world_width,
			world_height: self.world_height,
			width: (self.width.start + (width as f32 * width_clamp.start) as u32)..(self.width.start +  (width as f32 * width_clamp.end) as u32),
			height: (self.height.start + (height as f32 * height_clamp.start) as u32)..(self.height.start + (height as f32 * height_clamp.end) as u32)
		}
	}

	pub fn noise_x(&self, x: u32) -> f32 {
		x as f32
	}

	pub fn noise_y(&self, y: u32) -> f32 {
		y as f32
	}

	pub fn area_x(&self, x: u32) -> f32 {
		(x - self.width.start) as f32 / self.width() as f32
	}

	pub fn area_y(&self, y: u32) -> f32 {
		(y - self.height.start) as f32 / self.height() as f32
	}

	pub fn x(&self) -> u32 {
		self.width.start
	}

	pub fn y(&self) -> u32 {
		self.height.start
	}

	pub fn width(&self) -> u32 {
		self.width.end - self.width.start
	}

	pub fn height(&self) -> u32 {
		self.height.end - self.height.start
	}

	pub fn iter(self, mut func: impl FnMut(&Self, u32, u32)) {
		for y in self.height.clone() {
			for x in self.width.clone() {
				func(&self, x, y);
			}
		}
	}
}
