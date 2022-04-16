//! Ok can we please stop calling everything handler. There are a ton of conlicts.
//! Here are the definitions
//!
//! # Naming
//! When naming a handler or manager. Its always {singular}{type}, so if you are making a handler for networking.
//! Its called NetworkHandler.
//!
//! SUB LIBS SHOULD NEVER NAME ANYTHING A HANDLER OR A MANAGER.
//! ## Handlers
//! Handler are for the client and are a module of logic. Normally wraps one of our libraries.
//! ## Managers
//! Manager are the same as handlers. But for the server.

use std::net::SocketAddr;
use std::sync::Arc;

use eyre::{Result, WrapErr};
use mlua::Thread;
use rayon::{ThreadPool, ThreadPoolBuilder};

use manager::chunk::ChunkManager;
use manager::entity::EntityManager;
use manager::network::NetworkManager;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_network::{EstablishingInstance, NetworkInterface, Token};
use std::fmt::Display;

use crate::network::join::PlayerJoinData;
use crate::network::packet::{ClientPacket, ServerPacket};

pub mod api;
pub mod chunk;
pub mod entity;
mod manager;
pub mod network;

pub const UPS: usize = 20;

/// The main data structure for a server.
/// This is where the world is stored and the information gets distributed across clients.
pub struct Server {
    network: NetworkManager,
    chunk: ChunkManager,
    entity: EntityManager,
}

impl Server {
    pub fn new(thread_pool: Arc<ThreadPool>, ip_address: Option<SocketAddr>) -> Result<Server> {
        Ok(Server {
            network: NetworkManager::new(ServerNetworking::new(ip_address)?),
            chunk: ChunkManager::new(thread_pool.clone()),
            entity: EntityManager::new(thread_pool.clone()),
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        // yes i know there is unsafe here. Check the _todo in poll.
        self.network
            .internal
            .poll(unsafe { (self as *const Server as *mut Server).as_mut().unwrap() });
        self.chunk.tick(&mut self.network).wrap_err("Chunk error")?;
        self.entity
            .tick(&mut self.network)
            .wrap_err("Entity error")?;
        self.network.tick().wrap_err("Networking error")?;
        Ok(())
    }

    pub fn create_local_connection(&mut self) -> ClientNetworking<ServerPacket, ClientPacket> {
        ClientNetworking::join_local(&mut self.network.internal)
    }
}

impl NetworkInterface<ClientPacket, ServerPacket, PlayerJoinData> for Server {
    fn receive(&mut self, from: Token, packet: ClientPacket) {
        match packet {
            ClientPacket::Chunk(packet) => self.chunk.packet(from, packet),
            // TODO error handling here
            ClientPacket::Entity(packet) => self.entity.packet(from, packet).unwrap(),
        }
    }

    fn disconnected(&mut self, _client: Token) {}

    fn connected(&mut self, _client: Token, _connection_data: PlayerJoinData) {}

    fn establishing(&mut self) -> Box<dyn EstablishingInstance<PlayerJoinData>> {
        todo!()
    }
}

impl Reloadable for Server {
    fn reload(&mut self, api: &Api, carrier: &Carrier) {
        self.chunk.reload(api, carrier);
        self.entity.reload(api, carrier);
    }
}

#[derive(Debug)]
pub enum SmartError {
    CarrierUnavailable,
}

impl Display for SmartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartError::CarrierUnavailable => {
                f.write_str("Carrier is unavailable, Force reloading instance.")
            }
        }
    }
}
