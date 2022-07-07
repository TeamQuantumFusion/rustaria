use rsa_core::math::Vector2D;
use rsa_core::ty::WS;

pub mod packet;

#[derive(Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerCommand {
	pub dir: Vector2D<f32, WS>,
	pub jumping: bool,
}

#[derive(Clone)]
pub struct Player {
	pub pos: Vector2D<f32, WS>,
	pub velocity: Vector2D<f32, WS>,
}

impl Player {
	pub fn tick(&mut self, delta: f32) { self.pos += self.velocity * delta; }
}
