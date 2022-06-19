use euclid::{rect, Rect, Vector2D};
use rustaria::ty::WS;

use crate::Frontend;

#[derive(Copy, Clone)]
pub struct Viewport {
	pub pos: Vector2D<f32, WS>,
	pub zoom: f32,
	pub rect: Rect<f32, WS>,
}

impl Viewport {
	pub fn new(pos: Vector2D<f32, WS>, zoom: f32) -> Viewport {
		let mut viewport = Viewport {
			pos,
			zoom,
			rect: Rect::zero(),
		};
		viewport.recompute_rect(None);
		viewport
	}

	pub fn recompute_rect(&mut self, frontend: Option<&Frontend>) {
		let w = self.zoom / frontend.map(|f| f.aspect_ratio).unwrap_or(1.0);
		let h = self.zoom;
		self.rect = rect(self.pos.x - w, self.pos.y - h, w * 2.0, h * 2.0);
	}
}
