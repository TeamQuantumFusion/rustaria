#![allow(clippy::new_without_default)]

//! Ok can we please stop calling everything handler. There are a ton of conlicts.
//! Here are the definitions
//!
//! # Naming
//! When naming a handler or module. Its always {singular}{type}, so if you are making a handler for networking.
//! Its called NetworkHandler.
//!
//! SUB LIBS SHOULD NEVER NAME ANYTHING A HANDLER OR A SYSTEM.
//! ## Handlers
//! Handler are for the client and are a module of logic. Normally wraps one of our libraries.
//! ## Systems
//! Systems are the same as handlers. But for the server.

use std::fmt::{Debug, Display};
use std::net::SocketAddr;
use std::sync::Arc;

use rayon::ThreadPool;

use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_common::error::{Result, WrapErr};
use rustaria_network::server::integrated::Integrated;

// Internals
use crate::module::chunks::ChunkSystem;
use crate::module::entities::EntitySystem;
use crate::module::networking::NetworkSystem;
use crate::module::players::PlayerSystem;
use crate::packet::{ClientPacket, ServerPacket};

pub mod api;
pub mod chunk;
pub mod entity;
pub(crate) mod module;
pub mod packet;
pub mod player;
pub mod tile;
pub mod util;

pub type ServerNetwork = rustaria_network::server::ServerNetwork<ClientPacket, ServerPacket>;
pub type ClientNetwork = rustaria_network::client::ClientNetwork<ServerPacket, ClientPacket>;

/// The main object structure for a server.
/// This is where the world is stored and the information gets distributed across clients.
pub struct Server {
	pub api: Api,
	pub network: NetworkSystem,
	pub chunk: ChunkSystem,
	pub entity: EntitySystem,
	pub player: PlayerSystem,
}

impl Server {
	pub fn new(
		api: &Api,
		thread_pool: Arc<ThreadPool>,
	) -> Result<Server> {
		Ok(Server {
			api: api.clone(),
			network: NetworkSystem::new(ServerNetwork {
				integrated: Some(Integrated::new()?),
				remote: None
			}),
			chunk: ChunkSystem::new(thread_pool),
			entity: EntitySystem::new(),
			player: PlayerSystem::new(),
		})
	}

	//noinspection ALL
	pub fn tick(&mut self) -> Result<()> {
		self.api.invoke_hook("rustaria:tick", || ())?;
		ChunkSystem::tick(self).wrap_err(SmartError::SystemFailure(SystemType::Chunk))?;
		EntitySystem::tick(self).wrap_err(SmartError::SystemFailure(SystemType::Entity))?;
		NetworkSystem::tick(self).wrap_err(SmartError::SystemFailure(SystemType::Network))?;
		Ok(())
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
	SystemFailure(SystemType),
}

impl Display for SmartError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SmartError::CarrierUnavailable => {
				f.write_str("Carrier is unavailable, Force reloading instance.")
			}
			SmartError::SystemFailure(name) => {
				name.fmt(f)?;
				f.write_str(" system failure.")
			}
		}
	}
}

#[derive(Debug)]
pub enum SystemType {
	Entity,
	Chunk,
	Network,
	Player,
}
