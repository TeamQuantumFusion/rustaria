use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::chunk::Chunk;
use crate::packet::chunk::{ChunkBundlePacket, ServerChunkPacket};
use crate::packet::ServerPacket;
use crate::ServerNetwork;
use rustaria_network::packet::CompressedPacket;
use rustaria_network::Token;
use rustaria_util::ty::ChunkPos;

/// The `NetworkManager` handles networking for the server.
pub(crate) struct NetworkManager {
	internal: ServerNetwork,
	chunk_buffer: HashMap<Option<Token>, HashMap<ChunkPos, Chunk>>,
}

// TODO positional api, basically only send stuff if the player is nearby.
impl NetworkManager {
	pub fn new(networking: ServerNetwork) -> NetworkManager {
		NetworkManager {
			internal: networking,
			chunk_buffer: Default::default(),
		}
	}

	pub fn send_chunk(&mut self, to: Option<Token>, pos: ChunkPos, chunk: Chunk) {
		self.chunk_buffer.entry(to).or_insert_with(HashMap::new);
		self.chunk_buffer.get_mut(&to).unwrap().insert(pos, chunk);
	}

	pub fn tick(&mut self) -> rustaria_network::Result<()> {
		for (to, chunks) in self.chunk_buffer.drain() {
			let packet = ServerPacket::Chunk(ServerChunkPacket::Provide(CompressedPacket::new(
				&ChunkBundlePacket {
					chunks: chunks.into_iter().collect(),
				},
			)?));

			if let Some(to) = to {
				self.internal.send(to, packet)?
			} else {
				self.internal.send_all(packet)?;
			}
		}
		Ok(())
	}
}

impl Deref for NetworkManager {
	type Target = ServerNetwork;

	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl DerefMut for NetworkManager {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.internal
	}
}
