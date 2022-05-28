use serde::{Deserialize, Serialize};
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::{RawId, Uuid};


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
