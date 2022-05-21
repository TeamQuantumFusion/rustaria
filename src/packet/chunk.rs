use serde::{Deserialize, Serialize};

use rustaria_common::ty::ChunkPos;
use rustaria_network::packet::compress::Compress;

use crate::chunk::Chunk;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerChunkPacket {
	Provide(Compress<ChunkBundlePacket>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientChunkPacket {
	Request(Vec<ChunkPos>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkBundlePacket {
	pub chunks: Vec<(ChunkPos, Chunk)>,
}
