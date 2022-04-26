use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use rayon::ThreadPool;

use rustaria_api::{Carrier, Reloadable};
use rustaria_network::Token;
use rustaria_util::error::Result;
use rustaria_util::ty::ChunkPos;

use crate::chunk::ChunkStorage;
use crate::internal::chunks::world_generation::WorldGeneration;
use crate::NetworkManager;
use crate::packet::chunk::ClientChunkPacket;

mod world_generation;

pub(crate) struct ChunkManager {
	generator: WorldGeneration,
	storage: ChunkStorage,
	chunk_queue: VecDeque<(ChunkPos, Token)>,
	chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
	// Chunks that updated and need to be resent
	dirty_chunks: HashSet<ChunkPos>,
}

impl ChunkManager {
	pub fn new(thread_pool: Arc<ThreadPool>) -> ChunkManager {
		ChunkManager {
			generator: WorldGeneration::new(thread_pool).unwrap(),
			storage: Default::default(),
			chunk_queue: Default::default(),
			chunk_gen_queue: Default::default(),
			dirty_chunks: Default::default(),
		}
	}

	pub fn tick(&mut self, network: &mut NetworkManager) -> Result<()> {
		for (pos, from) in self.chunk_queue.drain(..) {
			if let Some(chunk) = self.storage.get_chunk(pos) {
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

			self.storage.put_chunk(pos, chunk);
		});

		for pos in self.dirty_chunks.drain() {
			if let Some(chunk) = self.storage.get_chunk(pos) {
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

impl Deref for ChunkManager {
	type Target = ChunkStorage;

	fn deref(&self) -> &Self::Target {
		&self.storage
	}
}

impl DerefMut for ChunkManager {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.storage
	}
}
