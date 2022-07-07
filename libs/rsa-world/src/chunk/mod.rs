use std::ops::{Index, IndexMut};

use apollo::{macros::*, FromLua, ToLua, UserData};
use block::Block;
use layer::BlockLayer;
use rsa_core::ty::IdTable;
use crate::CHUNK_SIZE;
use crate::ty::BlockLayerPos;


pub mod block;
pub mod layer;
pub mod storage;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
	pub layers: IdTable<BlockLayer, ChunkLayer<Block>>,
}

#[lua_impl]
impl Chunk {
	#[lua_method]
	pub fn get_layers(&self) -> &IdTable<BlockLayer, ChunkLayer<Block>> { &self.layers }

	#[lua_method]
	pub fn get_mut_layers(&mut self) -> &mut IdTable<BlockLayer, ChunkLayer<Block>> {
		&mut self.layers
	}
}

// Layer
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkLayer<T: Clone> {
	pub data: [[T; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<T: Clone> ChunkLayer<T> {
	pub fn entries(&self, mut func: impl FnMut(BlockLayerPos, &T)) {
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				func(BlockLayerPos::new(x as u8, y as u8), &self.data[y][x]);
			}
		}
	}

	pub fn entries_mut(&mut self, mut func: impl FnMut(BlockLayerPos, &mut T)) {
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				func(BlockLayerPos::new(x as u8, y as u8), &mut self.data[y][x]);
			}
		}
	}

	pub fn map<O: Clone + Copy>(
		&self,
		default: O,
		mut func: impl FnMut(&T) -> Option<O>,
	) -> ChunkLayer<O> {
		let mut out = ChunkLayer::new_copy(default);
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				if let Some(value) = func(&self.data[y][x]) {
					out.data[y][x] = value;
				}
			}
		}

		out
	}
}

#[lua_impl]
impl<T: UserData + Clone + Send + ToLua + FromLua + 'static> ChunkLayer<T> {
	#[lua_method(__index)]
	fn __index(&self, pos: BlockLayerPos) -> &T { &self[pos] }

	#[lua_method(__index)]
	fn __index_mut(&mut self, pos: BlockLayerPos) -> &mut T { &mut self[pos] }

	#[lua_method]
	fn set_entry(&mut self, pos: BlockLayerPos, value: T) { self[pos] = value; }
}

impl<T: Clone + Copy> ChunkLayer<T> {
	pub fn new_copy(value: T) -> Self {
		ChunkLayer {
			data: [[value; CHUNK_SIZE]; CHUNK_SIZE],
		}
	}
}

impl<T: Clone> Index<BlockLayerPos> for ChunkLayer<T> {
	type Output = T;

	fn index(&self, index: BlockLayerPos) -> &Self::Output {
		&self.data[index.y() as usize][index.x() as usize]
	}
}

impl<T: Clone> IndexMut<BlockLayerPos> for ChunkLayer<T> {
	fn index_mut(&mut self, index: BlockLayerPos) -> &mut Self::Output {
		&mut self.data[index.y() as usize][index.x() as usize]
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromLua)]
pub enum ConnectionType {
	// air
	Isolated,
	// tiles
	Connected,
}
