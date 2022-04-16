use serde::{Deserialize, Serialize};

use rustaria_network::Packet;
use rustaria_util::ty::ChunkPos;

use crate::chunk::Chunk;

use self::{
    chunk::{ClientChunkPacket, ServerChunkPacket},
    entity::{ClientEntityPacket, ServerEntityPacket},
};

pub mod chunk;
pub mod entity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPacket {
    Chunk(ServerChunkPacket),
    Entity(ServerEntityPacket),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientPacket {
    Chunk(ClientChunkPacket),
    Entity(ClientEntityPacket),
}

impl Packet for ServerPacket {}

impl Packet for ClientPacket {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkBundlePacket {
    pub chunks: Vec<(ChunkPos, Chunk)>,
}
