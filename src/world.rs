use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use legion::Schedule;
use rayon::{ThreadPool, ThreadPoolBuilder};


use rustaria_network::{EstablishingInstance, NetworkInterface, Token};
use rustaria_util::Result;


use crate::{ClientPacket, Networking, ServerPacket};
use crate::api::Api;
use crate::network::join::PlayerJoinData;
use chunk::ChunkHandler;
use crate::world::entity::EntityHandler;


pub mod chunk;
pub mod entity;
pub mod gen;
pub mod tile;
mod executor;

pub struct World {
    pub chunks: ChunkHandler,
    pub entities: EntityHandler,
    thread_pool: Arc<ThreadPool>,
}

impl World {
    pub fn new(api: Api, num_threads: usize) -> Result<World> {
        let thread_pool = Arc::new(ThreadPoolBuilder::new().num_threads(num_threads).build()?);

        Ok(World {
            chunks: ChunkHandler::new(&api, thread_pool.clone()),
            entities: EntityHandler::new(&api,thread_pool.clone()),
            thread_pool,
        })
    }

    pub fn tick(&mut self, network: &mut Networking) -> Result<()> {
        network.internal.poll(self);
        self.chunks.tick(network);
        self.entities.tick(network);
        network.tick()?;
        Ok(())
    }
}

impl NetworkInterface<ClientPacket, ServerPacket, PlayerJoinData> for World {
    fn receive(&mut self, from: Token, packet: ClientPacket) {
        match packet {
            ClientPacket::RequestChunks(chunks) => {
                self.chunks.client_requested(from, chunks);
            }
        }
    }

    fn disconnected(&mut self, _client: Token) {}

    fn connected(&mut self, _client: Token, _connection_data: PlayerJoinData) {}

    fn establishing(&mut self) -> Box<dyn EstablishingInstance<PlayerJoinData>> {
        todo!()
    }
}
