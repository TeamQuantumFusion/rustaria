use rsa_client_core::atlas::Atlas;
use rsa_core::math::{Rect, vec2, Vector2D};
use rsa_core::std::FxHashMap;
use crate::{Gui, GuiDrawer};
use crate::drawer::Fill;

pub struct GuiFonts {
	glyphs: FxHashMap<char, Glyph>,
	missing: Glyph,
}

impl GuiFonts {
	pub fn get_glyph(&self, ch: char) -> &Glyph {
		self.glyphs.get(&ch).unwrap_or(&self.missing)
	}

	pub fn get_size(&self, text: &str) -> Rect<f32, Gui> {
		let mut rect: Rect<f32, Gui> = Rect::zero();
		let mut offset = 0.0;
		for ch in text.chars() {
			let glyph = self.get_glyph(ch);
			let size = glyph.size.translate(vec2(offset, 0.0));
			let min_x = rect.min().min(size.min());
			let max_x = rect.max().max(size.max());
			rect = Rect::from_points([min_x, max_x]);
			offset += glyph.stride;
		}
		rect
	}
}

pub struct Glyph {
	pub size: Rect<f32, Gui>,
	pub stride: f32,
	pub atlas_location: Fill
}