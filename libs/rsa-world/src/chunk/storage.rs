use std::collections::hash_set::Iter;

use apollo::macros::*;
use rsa_core::std::{FxHashMap, FxHashSet};

use crate::{Chunk, ChunkPos};

#[derive(Clone)]
pub struct ChunkStorage {
	width: u32,
	height: u32,
	chunks: FxHashMap<ChunkPos, Chunk>,
	dirty: FxHashSet<ChunkPos>,
}

#[lua_impl]
impl ChunkStorage {
	pub fn new(width: u32, height: u32) -> ChunkStorage {
		ChunkStorage {
			width,
			height,
			chunks: Default::default(),
			dirty: Default::default(),
		}
	}

	#[lua_field(get width)]
	pub fn width(&self) -> u32 { self.width }

	#[lua_field(get height)]
	pub fn height(&self) -> u32 { self.height }

	#[lua_method]
	pub fn contains(&self, pos: ChunkPos) -> bool {
		if !self.check_inbounds(pos) {
			return false;
		}

		self.chunks.contains_key(&pos)
	}

	#[lua_method]
	pub fn get(&self, pos: ChunkPos) -> Option<&Chunk> {
		if !self.check_inbounds(pos) {
			return None;
		}

		self.chunks.get(&pos)
	}

	#[lua_method]
	pub fn get_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
		if !self.check_inbounds(pos) {
			return None;
		}

		self.dirty.insert(pos);
		self.chunks.get_mut(&pos)
	}

	#[lua_method]
	pub fn insert(&mut self, pos: ChunkPos, chunk: Chunk) -> Option<Chunk> {
		if !self.check_inbounds(pos) {
			return None;
		}

		self.dirty.insert(pos);
		self.chunks.insert(pos, chunk)
	}

	pub fn get_dirty(&self) -> Iter<'_, ChunkPos> { self.dirty.iter() }

	pub fn reset_dirty(&mut self) { self.dirty.clear(); }

	pub fn reset(&mut self) {
		self.reset_dirty();
		self.chunks.clear();
	}

	#[inline(always)]
	fn check_inbounds(&self, pos: ChunkPos) -> bool { pos.x < self.width && pos.y < self.height }
}
