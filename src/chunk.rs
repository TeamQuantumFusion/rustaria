use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use rustaria_common::ty::{ChunkPos, ChunkSubPos, CHUNK_SIZE};

use crate::tile::Tile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
	pub tiles: ChunkLayer<Tile>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkLayer<T> {
	pub grid: [[T; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<T> ChunkLayer<T> {
	pub fn new(values: [[T; CHUNK_SIZE]; CHUNK_SIZE]) -> ChunkLayer<T> {
		ChunkLayer { grid: values }
	}
}

impl<T> Index<ChunkSubPos> for ChunkLayer<T> {
	type Output = T;

	fn index(&self, index: ChunkSubPos) -> &Self::Output {
		&self.grid[index.y() as usize][index.x() as usize]
	}
}

impl<T> IndexMut<ChunkSubPos> for ChunkLayer<T> {
	fn index_mut(&mut self, index: ChunkSubPos) -> &mut Self::Output {
		&mut self.grid[index.y() as usize][index.x() as usize]
	}
}

#[derive(Default)]
pub struct ChunkStorage {
	chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkStorage {
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
