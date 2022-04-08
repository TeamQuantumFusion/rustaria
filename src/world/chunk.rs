use typemap::{Key};
use rustaria_util::ty::{CHUNK_SIZE, ChunkSubPos};
use serde::{Serialize, Deserialize};
use crate::world::tile::Tile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
	pub tiles: ChunkLayer<Tile>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkLayer<T: Copy + Clone> {
	values: [T; CHUNK_SIZE]
}

impl<T: Copy + Clone> ChunkLayer<T> {
	pub fn new(default: T) -> ChunkLayer<T>{
		ChunkLayer {
			values: [default; CHUNK_SIZE]
		}
	}

	#[inline(always)]
	pub fn get(&self, pos: ChunkSubPos) -> &T {
		&self.values[pos.index()]
	}

	#[inline(always)]
	pub fn get_mut(&mut self, pos: ChunkSubPos) -> &T {
		&mut self.values[pos.index()]
	}

	#[inline(always)]
	pub fn put(&mut self, value: T, pos: ChunkSubPos) {
		self.values[pos.index()] = value;
	}
}

impl<T: 'static + Copy + Clone> Key for ChunkLayer<T> {
	type Value = ChunkLayer<T>;
}
