use rustaria_common::logging::info;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use rustaria_common::ty::ChunkPos;
use rustaria_common::error::Result;
use rustaria_network::packet::compress::Compress;
use rustaria_network::Token;

use crate::chunk::Chunk;
use crate::packet::chunk::{ChunkBundlePacket, ServerChunkPacket};
use crate::packet::ServerPacket;
use crate::{ClientPacket, Server, ServerNetwork};

/// The `NetworkManager` handles networking for the server.
pub struct NetworkSystem {
	internal: ServerNetwork,
	chunk_buffer: HashMap<Option<Token>, HashMap<ChunkPos, Chunk>>,
}

// TODO positional api, basically only send stuff if the player is nearby.
impl NetworkSystem {
	pub fn new(networking: ServerNetwork) -> NetworkSystem {
		NetworkSystem {
			internal: networking,
			chunk_buffer: Default::default(),
		}
	}

	pub fn send_chunk(&mut self, to: Option<Token>, pos: ChunkPos, chunk: Chunk) {
		self.chunk_buffer.entry(to).or_insert_with(HashMap::new);
		self.chunk_buffer.get_mut(&to).unwrap().insert(pos, chunk);
	}

	#[macro_module::module(server.network)]
	pub fn tick(this: &mut NetworkSystem, server: &mut Server) -> Result<()> {
		for (to, chunks) in this.chunk_buffer.drain() {
			let packet = ServerPacket::Chunk(ServerChunkPacket::Provide(Compress::new(
				&ChunkBundlePacket {
					chunks: chunks.into_iter().collect(),
				},
			)?));

			if let Some(to) = to {
				this.internal.send(to, packet)?
			} else {
				this.internal.send_all(packet)?;
			}
		}

		let data = this.internal.tick()?;

		for token in data.to_connect {
			info!("{} connected", token);
			server.player.join(token);
		}

		for (from, packet) in data.received {
			match packet {
				ClientPacket::Chunk(packet) => server.chunk.packet(from, packet)?,
				ClientPacket::Player(packet) => {
					server
						.player
						.packet(from, packet, &mut server.entity, &server.network)?
				}
				ClientPacket::Entity(packet) => server.entity.packet(from, packet)?,
			}
		}

		for token in data.to_disconnect {
			info!("{} disconnected", token);
		}

		Ok(())
	}
}

impl Deref for NetworkSystem {
	type Target = ServerNetwork;

	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

impl DerefMut for NetworkSystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.internal
	}
}
