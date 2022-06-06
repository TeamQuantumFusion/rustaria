use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use rayon::ThreadPool;

use rsa_core::api::{Api, Reloadable};
use rsa_core::error::Result;
use rsa_core::ty::ChunkPos;
use rsa_network::Token;

use crate::module::chunks::world_generation::WorldGeneration;
use crate::packet::chunk::ClientChunkPacket;
use crate::Server;

mod world_generation;

pub struct ChunkModule {
	generator: WorldGeneration,
	chunk_queue: VecDeque<(ChunkPos, Token)>,
	chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
	// Chunks that updated and need to be resent
	dirty_chunks: HashSet<ChunkPos>,
}

impl ChunkModule {
	pub fn new(thread_pool: Arc<ThreadPool>) -> ChunkModule {
		ChunkModule {
			generator: WorldGeneration::new(thread_pool).unwrap(),
			chunk_queue: Default::default(),
			chunk_gen_queue: Default::default(),
			dirty_chunks: Default::default(),
		}
	}

	#[macro_module::module(server.chunk)]
	pub fn tick(this: &mut ChunkModule, server: &mut Server) -> Result<()> {
		for (pos, from) in this.chunk_queue.drain(..) {
			if let Some(chunk) = server.world.chunks.get_chunk(pos) {
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

			server.world.chunks.put_chunk(pos, chunk);
		});

		for pos in this.dirty_chunks.drain() {
			if let Some(chunk) = server.world.chunks.get_chunk(pos) {
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

	pub fn reload(&mut self, api: &Api) {
		self.generator.reload(api);
	}
}
