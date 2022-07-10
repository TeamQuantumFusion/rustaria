use apollo::{Function, FromLua};
use rsa_core::math::Rect;
use crate::drawer::{Fill, GuiDrawer};
use crate::Gui;
use crate::widget::Widget;

#[derive(FromLua)]
pub struct Button {
	pub text: String,
	pub pressed: Function,
	pub released: Function,
}

impl Widget for Button {
	fn get_size(&self, gui: &Gui)-> Rect<f32, Gui> {
		gui.fonts.get_size(&self.text).inflate(5.0, 5.0)
	}

	fn draw(&self, rect: Rect<f32, Gui>, gui: &mut Gui) {
		gui.drawer.draw_rect(rect, Fill::Color([255, 0, 0, 0]));
		gui.drawer.draw_text(rect.origin, &self.text, &gui.fonts);
	}
}