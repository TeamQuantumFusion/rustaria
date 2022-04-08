use crate::vertex::ty::Color;

pub struct AtlasImage<'a> {
	pub atlas: &'a Atlas,
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

pub struct Atlas {
	width: u32,
	height: u32,
}

impl Atlas {
	pub fn vertex_x(&self, value: u32) -> f32 {
		value as f32 / self.width as f32
	}

	pub fn vertex_y(&self, value: u32) -> f32 {
		value as f32 / self.height as f32
	}
}

pub struct Light {
	pub bl: Color,
	pub tl: Color,
	pub tr: Color,
	pub br: Color,
}