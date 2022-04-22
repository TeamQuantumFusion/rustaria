use serde::{Deserialize, Serialize};

use rustaria_api::ty::RawId;
use rustaria_util::math::{Vector2D, WorldSpace};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerEntityPacket {
	New(RawId, Vector2D<f32, WorldSpace>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientEntityPacket {
	Spawn(RawId, Vector2D<f32, WorldSpace>),
}
