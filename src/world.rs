use std::collections::{HashMap, HashSet, VecDeque};

use chunk::Chunk;
use rustaria_network::{EstablishingInstance, NetworkInterface, Token};
use rustaria_util::{Result};
use rustaria_util::ty::ChunkPos;

use crate::{ClientPacket, Networking, ServerPacket};
use crate::api::Api;
use crate::network::join::PlayerJoinData;
use crate::world::gen::WorldGenerator;

pub mod chunk;
pub mod gen;
pub mod tile;

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    generator: WorldGenerator,
    changed_chunks: HashSet<ChunkPos>,
    chunk_queue: VecDeque<(ChunkPos, Token)>,
    chunk_gen_queue: HashMap<ChunkPos, HashSet<Token>>,
    _entities: hecs::World,
}

impl World {
    pub fn new(api: Api) -> Result<World> {
        Ok(World {
            chunks: Default::default(),
            generator: WorldGenerator::new(api, 8)?,
            changed_chunks: Default::default(),
            chunk_queue: Default::default(),
            chunk_gen_queue: Default::default(),
            _entities: Default::default(),
        })
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
        network.internal.poll(self);

        for (pos, from) in self.chunk_queue.drain(..) {
            if let Some(chunk) = self.chunks.get(&pos) {
                network.send_chunk(Some(from), pos, chunk.clone());
            } else {
                self.generator.request_chunk(pos);
                self.chunk_gen_queue.entry(pos).or_insert_with(HashSet::new);
                self.chunk_gen_queue.get_mut(&pos).unwrap().insert(from);
            }
        }

        self.generator.poll_chunks(
            |chunk, pos| {
                if let Some(targets) = self.chunk_gen_queue.remove(&pos) {
                    for to in targets {
                        network.send_chunk(Some(to), pos, chunk.clone());
                    }
                }

                self.chunks.insert(pos, chunk);
            },
        );

        for pos in self.changed_chunks.drain() {
            if let Some(chunk) = self.chunks.get(&pos) {
                network.send_chunk(None, pos, chunk.clone());
            }
        }

        network.tick()?;
        Ok(())
    }
}

impl NetworkInterface<ClientPacket, ServerPacket, PlayerJoinData> for World {
    fn receive(&mut self, from: Token, packet: ClientPacket) {
        match packet {
            ClientPacket::RequestChunks(chunks) => {
                for pos in chunks {
                    self.chunk_queue.push_back((pos, from));
                }
            }
        }
    }

    fn disconnected(&mut self, _client: Token) {}

    fn connected(&mut self, _client: Token, _connection_data: PlayerJoinData) {}

    fn establishing(&mut self) -> Box<dyn EstablishingInstance<PlayerJoinData>> {
        todo!()
    }
}
