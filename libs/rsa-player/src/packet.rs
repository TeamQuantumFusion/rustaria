use rsa_core::{math::Vector2D, ty::WS};
use rsa_core::ty::Id;
use rsa_world::chunk::block::BlockDesc;
use rsa_world::chunk::layer::BlockLayer;
use rsa_world::entity::Entity;
use rsa_world::ty::BlockPos;

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