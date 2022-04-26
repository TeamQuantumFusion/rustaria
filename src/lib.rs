#![allow(clippy::new_without_default)]

//! Ok can we please stop calling everything handler. There are a ton of conlicts.
//! Here are the definitions
//!
//! # Naming
//! When naming a handler or internal. Its always {singular}{type}, so if you are making a handler for networking.
//! Its called NetworkHandler.
//!
//! SUB LIBS SHOULD NEVER NAME ANYTHING A HANDLER OR A SYSTEM.
//! ## Handlers
//! Handler are for the client and are a module of logic. Normally wraps one of our libraries.
//! ## Systems
//! Systems are the same as handlers. But for the server.

use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;

use rayon::ThreadPool;

use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_common::error::{Result, WrapErr};
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_network::{EstablishingInstance, NetworkInterface, Token};

// Internals
use crate::internal::chunks::ChunkSystem;
use crate::internal::entities::EntitySystem;
use crate::internal::networking::NetworkSystem;
use crate::internal::players::PlayerSystem;
use crate::packet::{ClientPacket, PlayerJoinData, ServerPacket};

pub mod api;
pub mod chunk;
pub mod entity;
pub(crate) mod internal;
pub mod packet;
pub mod player;
pub mod tile;
pub mod util;

pub const UPS: u64 = 20;

pub type ServerNetwork = ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>;
pub type ClientNetwork = ClientNetworking<ServerPacket, ClientPacket>;

/// The main object structure for a server.
/// This is where the world is stored and the information gets distributed across clients.
pub struct Server {
	api: Api,
	network: NetworkSystem,
	chunk: ChunkSystem,
	entity: EntitySystem,
	player: PlayerSystem,
}

impl Server {
	pub fn new(
		api: &Api,
		thread_pool: Arc<ThreadPool>,
		ip_address: Option<SocketAddr>,
	) -> Result<Server> {
		Ok(Server {
			api: api.clone(),
			network: NetworkSystem::new(ServerNetworking::new(ip_address)?),
			chunk: ChunkSystem::new(thread_pool),
			entity: EntitySystem::new(),
			player: PlayerSystem::new(),
		})
	}

	pub fn tick(&mut self) -> Result<()> {
		// yes i know there is unsafe here. Check the _todo in poll.
		{
			let interface = unsafe { (self as *const Server as *mut Server).as_mut().unwrap() };
			self.network.poll(interface);
		}

		self.api.invoke_hook("rustaria:tick", || ())?;

		self.chunk.tick(&mut self.network).wrap_err("Chunk error")?;
		self.entity
			.tick(&self.chunk, &mut self.network)
			.wrap_err("Entity error")?;
		self.network.tick().wrap_err("Networking error")?;
		Ok(())
	}

	pub fn create_local_connection(
		&mut self,
		join_data: PlayerJoinData,
	) -> ClientNetworking<ServerPacket, ClientPacket> {
		ClientNetworking::join_local(&mut self.network, join_data)
	}
}

impl NetworkInterface<ClientPacket, ServerPacket, PlayerJoinData> for Server {
	// TODO error handling here

	fn receive(&mut self, from: Token, packet: ClientPacket) {
		match packet {
			ClientPacket::Chunk(packet) => self.chunk.packet(from, packet),
			ClientPacket::Player(packet) => self
				.player
				.packet(from, packet, &mut self.entity, &self.network)
				.unwrap(),
			ClientPacket::Entity(packet) => self.entity.packet(from, packet).unwrap(),
		}
	}

	fn disconnected(&mut self, _client: Token) {}

	fn connected(&mut self, client: Token, data: PlayerJoinData) {
		self.player.join(client, data);
	}

	fn establishing(&mut self) -> Box<dyn EstablishingInstance<PlayerJoinData>> {
		todo!()
	}
}

impl Reloadable for Server {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.chunk.reload(api, carrier);
		self.player.reload(api, carrier);
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
