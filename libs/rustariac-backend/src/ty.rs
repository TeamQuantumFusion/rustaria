use rustaria_util::math::{AtlasSpace, rect, Rect, WorldSpace};

use crate::builder::Quadable;

pub type AtlasLocation = Rect<f32, AtlasSpace>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosTexture {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

impl<S> Quadable<PosTexture> for (Rect<f32, S>, Rect<f32, AtlasSpace>) {
	fn expand(self) -> [PosTexture; 4] {
		[
			PosTexture {
				position: [self.0.min_x(), self.0.min_y()],
				tex_coords: [self.1.min_x(), self.1.min_y()],
			},
			PosTexture {
				position: [self.0.min_x(), self.0.max_y()],
				tex_coords: [self.1.min_x(), self.1.max_y()],
			},
			PosTexture {
				position: [self.0.max_x(), self.0.max_y()],
				tex_coords: [self.1.max_x(), self.1.max_y()],
			},
			PosTexture {
				position: [self.0.max_x(), self.0.min_y()],
				tex_coords: [self.1.max_x(), self.1.min_y()],
			},
		]
	}
}

#[derive(Debug)]
pub struct Camera {
	pub position: [f32; 2],
	pub velocity: [f32; 2],
	pub zoom: f32,
	pub screen_y_ratio: f32,
}

impl Camera {
	pub fn visible(&self) -> Rect<f32, WorldSpace> {
		let x_view = self.zoom;
		let y_view = self.zoom / self.screen_y_ratio;
		rect(
			self.position[0] - x_view,
			self.position[1] - y_view,
			x_view * 2.0,
			y_view * 2.0,
		)
	}
}
