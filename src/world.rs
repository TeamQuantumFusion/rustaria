use std::sync::Arc;

use eyre::{Result, Context};
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::network::join::PlayerJoinData;
use crate::world::entity::EntityHandler;
use crate::{ClientPacket, Networking, ServerPacket};
use chunk::ChunkHandler;
use rustaria_api::{Carrier, Reloadable};
use rustaria_network::{EstablishingInstance, NetworkInterface, Token};
use rustaria_util::debug;

pub mod chunk;
pub mod entity;
pub mod gen;
pub mod tile;

pub struct World {
    pub chunks: ChunkHandler,
    pub entities: EntityHandler,
    _thread_pool: Arc<ThreadPool>,
}

impl World {
    pub fn new(num_threads: usize) -> Result<World> {
        let thread_pool = Arc::new(ThreadPoolBuilder::new().num_threads(num_threads).build()?);

        Ok(World {
            chunks: ChunkHandler::new(thread_pool.clone()),
            entities: EntityHandler::new(thread_pool.clone()),
            _thread_pool: thread_pool,
        })
    }

    pub fn tick(&mut self, network: &mut Networking) -> Result<()> {
        network.internal.poll(self);
        self.chunks.tick(network).wrap_err("Chunk error")?;
        self.entities.tick(network);
        network.tick().wrap_err("Networking error")?;
        Ok(())
    }
}

impl Reloadable for World {
    fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
        debug!("Reloading World");
        self.chunks.reload(api, carrier);
        self.entities.reload(api, carrier);
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
