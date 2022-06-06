use rsa_network::packet::{Packet, PacketDesc};
use serde::{Deserialize, Serialize};
use crate::entity::packet::{ClientEntityPacket, ServerEntityPacket};
use crate::packet::chunk::{ClientChunkPacket, ServerChunkPacket};
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};

pub mod chunk;
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

impl Packet for ServerPacket {
	fn get_desc(&self) -> PacketDesc {
		todo!()
	}
}
impl Packet for ClientPacket {
	fn get_desc(&self) -> PacketDesc {
		todo!()
	}
}
