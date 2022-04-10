use std::collections::HashMap;

use rustaria_network::networking::ServerNetworking;
use rustaria_network::packet::CompressedPacket;
use rustaria_network::Token;
use rustaria_util::ty::ChunkPos;
use rustaria_util::Result;

use crate::network::join::PlayerJoinData;
use crate::network::packet::ChunkBundlePacket;
use crate::world::chunk::Chunk;
use crate::{ClientPacket, ServerPacket};

pub mod join;
pub mod packet;

pub struct Networking {
    pub internal: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>,
    chunk_buffer: HashMap<Option<Token>, HashMap<ChunkPos, Chunk>>,
}

impl Networking {
    pub fn new(networking: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>) -> Networking{
        Networking {
            internal: networking,
            chunk_buffer: Default::default()
        }
    }

    pub fn send_chunk(&mut self, to: Option<Token>, pos: ChunkPos, chunk: Chunk) {
        if !self.chunk_buffer.contains_key(&to) {
            self.chunk_buffer.insert(to, HashMap::new());
        }

        self.chunk_buffer.get_mut(&to).unwrap().insert(pos, chunk);
    }

    pub fn tick(&mut self) -> Result<()> {
        for (to, chunks) in self.chunk_buffer.drain() {
            let packet = ServerPacket::Chunks(CompressedPacket::new(&ChunkBundlePacket {
                chunks: chunks.into_iter().collect(),
            })?);

            if let Some(to) = to {
                self.internal.send(to, packet)?
            } else {
                self.internal.distribute(Token::nil(), packet)?;
            }
        }
        Ok(())
    }
}
