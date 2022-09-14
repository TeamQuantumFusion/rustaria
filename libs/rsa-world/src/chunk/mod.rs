use std::ops::{Index, IndexMut};

use apollo::{macros::*, FromLua, ToLua, UserData};
use block::Block;
use layer::ChunkLayerType;
use rsa_registry::Storage;

use crate::{ty::BlockLayerPos, CHUNK_SIZE};

pub mod block;
pub mod layer;
pub mod storage;

/// A Chunk is a 16x16 piece of the world which contains layers of blocks which are specific to that layer.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
	pub layers: Storage<ChunkLayer, ChunkLayerType>,
}

#[lua_impl]
impl Chunk {
	#[lua_method]
	pub fn set_block(&mut self, pos: BlockLayerPos, block: Block) {
		let layer = &mut self.layers[block.layer];
		layer[pos] = block;
	}
}

/// A chunk layer is a layer of a chunk which holds a 16x16 grid of blocks which are specific to the layer.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkLayer {
	pub data: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
}

#[lua_impl]
impl ChunkLayer {
	pub fn new(value: Block) -> Self {
		ChunkLayer {
			data: [[value; CHUNK_SIZE]; CHUNK_SIZE],
		}
	}

	pub fn entries(&self, mut func: impl FnMut(BlockLayerPos, &Block)) {
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				func(BlockLayerPos::new(x as u8, y as u8), &self.data[y][x]);
			}
		}
	}

	pub fn entries_mut(&mut self, mut func: impl FnMut(BlockLayerPos, &mut Block)) {
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				func(BlockLayerPos::new(x as u8, y as u8), &mut self.data[y][x]);
			}
		}
	}

	#[lua_method(__index)]
	fn __index(&self, pos: BlockLayerPos) -> &Block { &self[pos] }

	#[lua_method(__index)]
	fn __index_mut(&mut self, pos: BlockLayerPos) -> &mut Block { &mut self[pos] }

	#[lua_method]
	fn set_entry(&mut self, pos: BlockLayerPos, value: Block) { self[pos] = value; }
}

impl Index<BlockLayerPos> for ChunkLayer {
	type Output = Block;

	fn index(&self, index: BlockLayerPos) -> &Self::Output {
		&self.data[index.y() as usize][index.x() as usize]
	}
}

impl IndexMut<BlockLayerPos> for ChunkLayer {
	fn index_mut(&mut self, index: BlockLayerPos) -> &mut Self::Output {
		&mut self.data[index.y() as usize][index.x() as usize]
	}
}