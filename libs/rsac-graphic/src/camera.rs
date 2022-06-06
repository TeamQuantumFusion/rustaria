use rsa_core::math::{Vector2D, WorldSpace};

#[derive(Clone)]
pub struct Camera {
	pub pos: Vector2D<f32, WorldSpace>,
	pub scale: f32,
}