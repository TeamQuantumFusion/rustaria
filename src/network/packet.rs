use serde::{Deserialize, Serialize};

use crate::world::chunk::Chunk;
use rustaria_network::packet::CompressedPacket;
use rustaria_network::Packet;
use rustaria_util::ty::ChunkPos;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPacket {
    Chunks(CompressedPacket<ChunkBundlePacket>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientPacket {
    RequestChunks(Vec<ChunkPos>),
}

impl Packet for ServerPacket {}

impl Packet for ClientPacket {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkBundlePacket {
    pub chunks: Vec<(ChunkPos, Chunk)>,
}
