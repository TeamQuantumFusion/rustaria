use rsa_core::{
	math::Vector2D,
	ty::WS,
};
use rsa_registry::Id;
use rsa_world::{
	chunk::layer::ChunkLayerType,
	entity::Entity,
	ty::BlockPos,
};
use rsa_world::chunk::block::ty::BlockType;

use crate::PlayerCommand;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerBoundPlayerPacket {
	SetMove(u32, PlayerCommand),
	PlaceBlock(BlockPos, Id<ChunkLayerType>, Id<BlockType>),
	Join(),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundPlayerPacket {
	RespondPos(u32, Option<Vector2D<f32, WS>>),
	Joined(Entity),
}
