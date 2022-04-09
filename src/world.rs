use std::collections::{HashMap, HashSet};

use chunk::Chunk;
use rustaria_network::packet::CompressedPacket;
use rustaria_network::{EstablishingInstance, NetworkInterface, Token};
use rustaria_util::ty::ChunkPos;
use rustaria_util::{debug, info, Result};

use crate::network::join::PlayerJoinData;
use crate::network::packet::ChunkBundlePacket;
use crate::{ClientPacket, Networking, ServerPacket};

pub mod chunk;
pub mod tile;

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    changed_chunks: HashSet<ChunkPos>,
    entities: hecs::World,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: Default::default(),
            changed_chunks: Default::default(),
            entities: Default::default(),
        }
    }

    pub fn put_chunk(&mut self, pos: ChunkPos, chunk: Chunk) {
        self.chunks.insert(pos, chunk);
        self.update_chunk(pos);
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }

    pub fn update_chunk(&mut self, pos: ChunkPos) {
        self.changed_chunks.insert(pos);
    }

    pub fn tick(&mut self, network: &mut Networking) -> Result<()> {
        let mut chunk_changed = Vec::new();
        for pos in self.changed_chunks.drain() {
            if let Some(chunk) = self.chunks.get(&pos) {
                chunk_changed.push((pos, chunk.clone()))
            }
        }

        if !chunk_changed.is_empty() {
            network.internal.distribute(
                Token::nil(),
                ServerPacket::Chunks(CompressedPacket::new(&ChunkBundlePacket {
                    chunks: chunk_changed,
                })?),
            )?;
        }

        Ok(())
    }
}

impl NetworkInterface<ClientPacket, ServerPacket, PlayerJoinData> for World {
    fn receive(&mut self, from: Token, packet: ClientPacket) {
        todo!()
    }

    fn disconnected(&mut self, client: Token) {
        todo!()
    }

    fn connected(&mut self, client: Token, connection_data: PlayerJoinData) {
        todo!()
    }

    fn establishing(&mut self) -> Box<dyn EstablishingInstance<PlayerJoinData>> {
        todo!()
    }
}
