use crate::builder::Quadable;
use rustaria_util::ty::Rectangle;

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
pub struct Camera {
	pub position: [f32; 2],
	pub zoom: f32,
	pub screen_y_ratio: f32,
}

impl Camera {
	pub fn visible(&self) -> Rectangle {
		let x_view = (self.zoom);
		let y_view = (self.zoom / self.screen_y_ratio);
		Rectangle {
			x: self.position[0] - x_view,
			y: self.position[1] - y_view,
			width: x_view * 2.0,
			height: y_view * 2.0,
		}
	}
}
