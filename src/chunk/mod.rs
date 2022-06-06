use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use layer::ChunkLayer;
use layer::tile::Tile;
use rsa_core::ty::ChunkPos;

pub mod layer;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Chunk {
	pub tiles: ChunkLayer<Tile>,
}

// Kinda empty for now
pub struct ChunkSystem {
	chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkSystem {
	pub fn new() -> ChunkSystem  {
		ChunkSystem {
			chunks: Default::default()
		}
	} 
	
	pub fn put_chunk(&mut self, pos: ChunkPos, chunk: Chunk) {
		self.chunks.insert(pos, chunk);
	}

	pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
		self.chunks.get(&pos)
	}

	pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
		self.chunks.get_mut(&pos)
	}

	pub fn clear(&mut self) {
		self.chunks.clear();
	}
}
