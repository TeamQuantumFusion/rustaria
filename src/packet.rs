use serde::{Deserialize, Serialize};

use rustaria_network::{EstablishingInstance, EstablishingStatus, Packet, Result};

use crate::packet::chunk::{ClientChunkPacket, ServerChunkPacket};
use crate::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use crate::player::Player;

pub mod chunk;
pub mod entity;
pub mod player;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPacket {
	Chunk(ServerChunkPacket),
	Entity(ServerEntityPacket),
	Player(ServerPlayerPacket),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientPacket {
	Chunk(ClientChunkPacket),
	Entity(ClientEntityPacket),
	Player(ClientPlayerPacket),
}

impl Packet for ServerPacket {}
impl Packet for ClientPacket {}

pub struct PlayerJoinInstance {}

impl EstablishingInstance<PlayerJoinData> for PlayerJoinInstance {
	fn receive(&mut self, _data: &[u8]) -> Result<EstablishingStatus<PlayerJoinData>> {
		todo!()
	}
}

pub struct PlayerJoinData {
	pub player: Player,
}
