mod layout;
mod button;

use rsa_client_core::ty::{MeshBuilder, PosTexVertex};
use rsa_core::math::default::Vector2D;
use rsa_core::math::Rect;
use crate::drawer::GuiDrawer;
use crate::Gui;

pub trait Widget {
	fn get_size(&self, gui: &Gui) -> Rect<f32, Gui>;
	fn draw(&self, rect: Rect<f32, Gui>, gui: &mut Gui);
}