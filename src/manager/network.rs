use std::collections::HashMap;

use rustaria_network::networking::ServerNetworking;
use rustaria_network::packet::CompressedPacket;
use rustaria_network::Token;
use rustaria_util::ty::ChunkPos;

use crate::chunk::Chunk;
use crate::network::packet::chunk::ServerChunkPacket;
use crate::network::packet::ChunkBundlePacket;
use crate::{ClientPacket, PlayerJoinData, ServerPacket};

/// The `NetworkManager` handles networking for the server.
pub(crate) struct NetworkManager {
	pub internal: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>,
	chunk_buffer: HashMap<Option<Token>, HashMap<ChunkPos, Chunk>>,
}

impl NetworkManager {
	pub fn new(
		networking: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>,
	) -> NetworkManager {
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
				self.internal.distribute(Token::nil(), packet)?;
			}
		}
		Ok(())
	}
}
