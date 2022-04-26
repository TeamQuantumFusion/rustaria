use serde::Deserialize;

use rustaria_common::math::{Vector2D, WorldSpace};

#[derive(Clone, Debug, Deserialize)]
pub struct PositionComp {
	pub position: Vector2D<f32, WorldSpace>,
}
