use serde::{Deserialize, Serialize};

use rustaria_api::ty::RawId;
use rustaria_util::math::{Vector2D, WorldSpace};
use rustaria_util::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerEntityPacket {
	New(Uuid, RawId, Vector2D<f32, WorldSpace>),
	Kill(Uuid),
	SetPos(Uuid, Vector2D<f32, WorldSpace>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientEntityPacket {
	Spawn(RawId, Vector2D<f32, WorldSpace>),
}
