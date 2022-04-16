use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use rustaria_api::{Carrier, Reloadable};
use rustaria_network::Token;
use rustaria_util::ty::ChunkPos;

use crate::chunk::Chunk;
use crate::manager::network::NetworkManager;
use crate::manager::world_gen::WorldGenManager;
use crate::network::packet::chunk::ClientChunkPacket;
use crate::ThreadPool;

pub(crate) struct ChunkManager {
	generator: WorldGenManager,
	chunks: HashMap<ChunkPos, Chunk>,
	chunk_queue: VecDeque<(ChunkPos, Token)>,
	chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
	// Chunks that updated and need to be resent
	dirty_chunks: HashSet<ChunkPos>,
}

impl ChunkManager {
	pub fn new(thread_pool: Arc<ThreadPool>) -> ChunkManager {
		ChunkManager {
			generator: WorldGenManager::new(thread_pool).unwrap(),
			chunks: Default::default(),
			chunk_queue: Default::default(),
			chunk_gen_queue: Default::default(),
			dirty_chunks: Default::default(),
		}
	}

	#[allow(unused)]
	pub fn put_chunk(&mut self, pos: ChunkPos, chunk: Chunk) {
		self.chunks.insert(pos, chunk);
		self.dirty_chunks.insert(pos);
	}

	#[allow(unused)]
	pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
		self.chunks.get(&pos)
	}

	#[allow(unused)]
	pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
		self.chunks.get_mut(&pos)
	}

	pub fn tick(&mut self, network: &mut NetworkManager) -> eyre::Result<()> {
		for (pos, from) in self.chunk_queue.drain(..) {
			if let Some(chunk) = self.chunks.get(&pos) {
				network.send_chunk(Some(from), pos, chunk.clone());
			} else {
				self.generator.request_chunk(pos)?;
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

		Ok(())
	}

	pub fn packet(&mut self, from: Token, packet: ClientChunkPacket) {
		match packet {
			ClientChunkPacket::Request(chunks) => {
				for pos in chunks {
					self.chunk_queue.push_back((pos, from));
				}
			}
		}
	}
}

impl Reloadable for ChunkManager {
	fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
		self.generator.reload(api, carrier);
	}
}
