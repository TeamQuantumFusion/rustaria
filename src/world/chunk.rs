use serde::{Deserialize, Serialize};

use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, ChunkSubPos};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use rayon::ThreadPool;
use rustaria_network::Token;
use crate::{Api, Networking};
use crate::world::gen::ChunkGenerator;

use crate::world::tile::Tile;

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
        ChunkLayer {
            grid: values,
        }
    }

    #[inline(always)]
    pub fn get(&self, pos: ChunkSubPos) -> &T {
        &self.grid[pos.y() as usize][pos.x() as usize]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: ChunkSubPos) -> &mut T {
        &mut self.grid[pos.y() as usize][pos.x() as usize]
    }

    #[inline(always)]
    pub fn put(&mut self, value: T, pos: ChunkSubPos) {
        self.grid[pos.y() as usize][pos.x() as usize] = value;
    }
}

pub struct ChunkHandler {
	generator: ChunkGenerator,
	chunks: HashMap<ChunkPos, Chunk>,
	chunk_queue: VecDeque<(ChunkPos, Token)>,
	chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
	dirty_chunks: HashSet<ChunkPos>,
}

impl ChunkHandler {
	pub fn new(api: &Api, thread_pool: Arc<ThreadPool>) -> ChunkHandler {
		ChunkHandler  {
			generator: ChunkGenerator::new(api.clone(), thread_pool).unwrap(),
			chunks: Default::default(),
			chunk_queue: Default::default(),
			chunk_gen_queue: Default::default(),
			dirty_chunks: Default::default(),
		}
	}
	pub fn put_chunk(&mut self, pos: ChunkPos, chunk: Chunk) {
		self.chunks.insert(pos, chunk);
		self.dirty_chunk(pos);
	}

	pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
		self.chunks.get(&pos)
	}

	pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
		self.chunks.get_mut(&pos)
	}

	pub fn dirty_chunk(&mut self, pos: ChunkPos) {
		self.dirty_chunks.insert(pos);
	}

	pub fn client_requested(&mut self, from: Token, chunks: Vec<ChunkPos>) {
		for pos in chunks {
			self.chunk_queue.push_back((pos, from));
		}
	}

	pub fn tick(&mut self, network: &mut Networking) {
		for (pos, from) in self.chunk_queue.drain(..) {
			if let Some(chunk) = self.chunks.get(&pos) {
				network.send_chunk(Some(from), pos, chunk.clone());
			} else {
				self.generator.request_chunk(pos);
				self.chunk_gen_queue.entry(pos).or_insert_with(HashSet::new);
				self.chunk_gen_queue.get_mut(&pos).unwrap().insert(from);
			}
		}

		self.generator.poll_chunks(|chunk, pos| {
			if let Some(targets) = self.chunk_gen_queue.remove(&pos) {
				for to in targets {
					network.send_chunk(Some(to), pos, chunk.clone());
				}
			}

			self.chunks.insert(pos, chunk);
		});

		for pos in self.dirty_chunks.drain() {
			if let Some(chunk) = self.chunks.get(&pos) {
				network.send_chunk(None, pos, chunk.clone());
			}
		}
	}
}

