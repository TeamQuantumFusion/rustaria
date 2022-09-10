use rsa_client_core::atlas::Atlas;
use rsa_core::math::{Point2D, Rect, Vector2D};
use crate::{Gui, GuiFonts};

pub struct GuiDrawer {

}

impl GuiDrawer {
	pub fn draw_rect(&mut self, rect: Rect<f32, Gui>, fill: Fill) {
		todo!()
	}

	pub fn draw_text(&mut self, pos: Point2D<f32, Gui>, text: &str, fonts: &GuiFonts) {
		todo!()
	}

}

pub enum Fill {
	Color([u8; 4]),
	Atlas(Rect<f32, Atlas>)
}