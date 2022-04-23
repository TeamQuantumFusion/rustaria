use rustaria_util::math::{Vector2D, WorldSpace};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PositionComp {
	pub position: Vector2D<f32, WorldSpace>,
}
