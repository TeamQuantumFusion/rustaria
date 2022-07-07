use rsa_core::{
	math::Vector2D,
	ty::{Id, WS},
};
use rsa_world::{
	chunk::{block::BlockDesc, layer::BlockLayer},
	entity::Entity,
	ty::BlockPos,
};

use crate::PlayerCommand;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerBoundPlayerPacket {
	SetMove(u32, PlayerCommand),
	PlaceBlock(BlockPos, Id<BlockLayer>, Id<BlockDesc>),
	Join(),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundPlayerPacket {
	RespondPos(u32, Option<Vector2D<f32, WS>>),
	Joined(Entity),
}
