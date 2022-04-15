use rustaria_api::ty::RawId;
use serde::{Deserialize, Serialize};

use rustaria_network::Packet;
use rustaria_network::packet::CompressedPacket;
use rustaria_util::ty::ChunkPos;
use rustaria_util::ty::pos::Pos;

use crate::world::chunk::Chunk;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPacket {
    Chunks(CompressedPacket<ChunkBundlePacket>),
    NewEntity(RawId, Pos)
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
