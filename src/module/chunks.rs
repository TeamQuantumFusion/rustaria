use std::cell::Ref;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use rayon::ThreadPool;

use rsa_core::api::{Api, Reloadable};
use rsa_core::error::Result;
use rsa_core::ty::ChunkPos;
use rsa_network::Token;

use crate::chunk::ChunkStorage;
use crate::module::chunks::world_generation::WorldGeneration;
use crate::packet::chunk::ClientChunkPacket;
use crate::Server;

mod world_generation;

pub struct ChunkSystem {
	generator: WorldGeneration,
	storage: ChunkStorage,
	chunk_queue: VecDeque<(ChunkPos, Token)>,
	chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
	// Chunks that updated and need to be resent
	dirty_chunks: HashSet<ChunkPos>,
}

impl ChunkSystem {
	pub fn new(thread_pool: Arc<ThreadPool>) -> ChunkSystem {
		ChunkSystem {
			generator: WorldGeneration::new(thread_pool).unwrap(),
			storage: Default::default(),
			chunk_queue: Default::default(),
			chunk_gen_queue: Default::default(),
			dirty_chunks: Default::default(),
		}
	}

	#[macro_module::module(server.chunk)]
	pub fn tick(this: &mut ChunkSystem, server: &mut Server) -> Result<()> {
		for (pos, from) in this.chunk_queue.drain(..) {
			if let Some(chunk) = this.storage.get_chunk(pos) {
				server.network.send_chunk(Some(from), pos, chunk.clone());
			} else {
				this.generator.request_chunk(pos)?;
				this.chunk_gen_queue.entry(pos).or_insert_with(HashSet::new);
				this.chunk_gen_queue.get_mut(&pos).unwrap().insert(from);
			}
		}

		this.generator.poll_chunks(|chunk, pos| {
			if let Some(targets) = this.chunk_gen_queue.remove(&pos) {
				for to in targets {
					server.network.send_chunk(Some(to), pos, chunk.clone());
				}
			}

			this.storage.put_chunk(pos, chunk);
		});

		for pos in this.dirty_chunks.drain() {
			if let Some(chunk) = this.storage.get_chunk(pos) {
				server.network.send_chunk(None, pos, chunk.clone());
			}
		}

		Ok(())
	}

	pub fn packet(&mut self, from: Token, packet: ClientChunkPacket) -> Result<()> {
		match packet {
			ClientChunkPacket::Request(chunks) => {
				for pos in chunks {
					self.chunk_queue.push_back((pos, from));
				}
			}
		}

		Ok(())
	}
}

impl Reloadable for ChunkSystem {
	fn reload(&mut self, api: &Api) {
		self.generator.reload(api);
	}
}

impl Deref for ChunkSystem {
	type Target = ChunkStorage;

	fn deref(&self) -> &Self::Target {
		&self.storage
	}
}

impl DerefMut for ChunkSystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.storage
	}
}
