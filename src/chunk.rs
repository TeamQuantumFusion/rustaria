use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use rustaria_api::ty::RawId;
use rustaria_util::ty::{ChunkSubPos, CHUNK_SIZE};

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tile {
	pub id: RawId,
	pub collision: bool,
	pub opaque: bool,
}
