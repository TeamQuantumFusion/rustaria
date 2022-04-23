use serde::{Deserialize, Serialize};

use crate::chunk::Chunk;
use rustaria_network::packet::CompressedPacket;
use rustaria_util::ty::ChunkPos;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerChunkPacket {
	Provide(CompressedPacket<ChunkBundlePacket>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientChunkPacket {
	Request(Vec<ChunkPos>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkBundlePacket {
	pub chunks: Vec<(ChunkPos, Chunk)>,
}
