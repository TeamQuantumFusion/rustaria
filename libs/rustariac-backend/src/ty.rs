use crate::builder::Quadable;

pub type AtlasLocation = Rectangle;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosTexture {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

impl Quadable<PosTexture> for (Rectangle, AtlasLocation) {
	fn expand(self) -> [PosTexture; 4] {
		[
			PosTexture {
				position: self.0.left_bottom(),
				tex_coords: self.1.left_bottom(),
			},
			PosTexture {
				position: self.0.left_top(),
				tex_coords: self.1.left_top(),
			},
			PosTexture {
				position: self.0.right_top(),
				tex_coords: self.1.right_top(),
			},
			PosTexture {
				position: self.0.right_bottom(),
				tex_coords: self.1.right_bottom(),
			},
		]
	}
}

#[derive(Debug)]
pub struct Viewport {
	pub position: [f32; 2],
	pub zoom: f32,
}

impl Viewport {
	pub fn viewport(&self, screen_y_ratio: f32) -> Rectangle {
		Rectangle {
			x: self.position[0] - ((self.zoom / 2.0) * screen_y_ratio),
			y: self.position[1] - (self.zoom / 2.0),
			width: self.zoom * screen_y_ratio,
			height: self.zoom,
		}
	}
}
#[derive(Copy, Clone)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rectangle {
	pub fn overlaps(&self, rect: &Rectangle) -> bool {
		self.left().max(rect.left()) < self.right().min(rect.right())
			&& self.bottom().max(rect.bottom()) < self.top().min(rect.top())
	}

	pub fn right_top(self) -> [f32; 2] {
		[self.right(), self.top()]
	}

	pub fn left_top(self) -> [f32; 2] {
		[self.left(), self.top()]
	}

	pub fn right_bottom(self) -> [f32; 2] {
		[self.right(), self.bottom()]
	}

	pub fn left_bottom(self) -> [f32; 2] {
		[self.left(), self.bottom()]
	}

	pub fn right(self) -> f32 {
		self.x + self.width
	}

	pub fn left(self) -> f32 {
		self.x
	}

	pub fn top(self) -> f32 {
		self.y + self.height
	}

	pub fn bottom(self) -> f32 {
		self.y
	}
}
