use rsa_core::math::{Rect, rect, Vector2D};
use rsa_core::ty::WS;

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
		viewport.recompute_rect(1.0);
		viewport
	}

	pub fn recompute_rect(&mut self, aspect_ratio: f32) {
		let w = self.zoom / aspect_ratio;
		let h = self.zoom;
		self.rect = rect(self.pos.x - w, self.pos.y - h, w * 2.0, h * 2.0);
	}
}
