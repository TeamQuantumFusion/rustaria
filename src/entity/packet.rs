use hecs::Entity;
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::RawId;
use crate::{ClientPacket, ServerPacket};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ClientEntityPacket {
	RequestPos(u32, Entity),
	// Humanoid
	PlayerDirection(u32, Vector2D<f32, WorldSpace>),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ServerEntityPacket {
	Pos(u32, Entity, Vector2D<f32, WorldSpace>),
	Spawn(u32, Entity, RawId),
}


impl Into<ServerPacket> for ServerEntityPacket {
	fn into(self) -> ServerPacket {
		ServerPacket::Entity(self)
	}
}

impl Into<ClientPacket> for ClientEntityPacket {
	fn into(self) -> ClientPacket {
		ClientPacket::Entity(self)
	}
}
