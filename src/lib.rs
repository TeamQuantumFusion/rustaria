#![allow(clippy::new_without_default)]

//! # Welcome!
//! Welcome to the rustaria codebase. Here you will find the code for the game Rustaria
//! which is a rework of Terraria which is designed for easy modality and speed.
//!
//! # Notice
//! 1. This is made in rust which may be a hard language to learn and write in.
//! We cannot help you learn rust as it is a difficult thing to do, if you want to contribute please learn the basics of rust beforehand.
//! 2. The comments in this project may contain swear words and other funnies.
//! This project permits the use of funny words along as it does not directly hurt an individual.
//! Saying "This code is fucking pain" is good, saying "fuck germans" is not.
//!

use std::fmt::{Debug, Display};
use std::sync::Arc;

use rayon::ThreadPool;

use rsa_core::api::Api;
use rsa_core::error::{Result, WrapErr};
use rsa_core::ty::Tag;
use rsa_network::server::integrated::Integrated;

// Internals
use crate::module::chunks::ChunkModule;
use crate::module::entities::EntityModule;
use crate::module::networking::NetworkModule;
use crate::module::players::PlayerModule;
use crate::packet::{ClientPacket, ServerPacket};
use crate::world::World;

pub mod api;
pub mod chunk;
pub mod entity;
pub(crate) mod module;
pub mod packet;
pub mod player;
pub mod util;
pub mod world;

pub type ServerNetwork = rsa_network::server::ServerNetwork<ClientPacket, ServerPacket>;
pub type ServerTunnel<'a, C> = rsa_network::tunnel::MappedTunnel<'a, ServerPacket, C>;
pub type ClientNetwork = rsa_network::client::ClientNetwork<ServerPacket, ClientPacket>;
pub type ClientTunnel<'a, C> = rsa_network::tunnel::MappedTunnel<'a, ClientPacket, C>;

/// The main object structure for a server.
/// This is where the world is stored and the information gets distributed across clients.
pub struct Server {
	pub api: Api,
	pub network: NetworkModule,
	pub chunk: ChunkModule,
	pub entity: EntityModule,
	pub player: PlayerModule,

	// Holds the actual data
	pub world: World,
}

impl Server {
	pub fn new_integrated(api: &Api, thread_pool: Arc<ThreadPool>) -> Result<Server> {
		Ok(Server {
			api: api.clone(),
			network: NetworkModule::new(ServerNetwork {
				integrated: Some(Integrated::new()?),
				remote: None,
			}),
			chunk: ChunkModule::new(thread_pool),
			entity: EntityModule::new(),
			player: PlayerModule::new(api),
			world: World::new(),
		})
	}

	//noinspection ALL
	pub fn tick(&mut self) -> Result<()> {
		// Receive
		NetworkModule::tick(self).wrap_err(SystemFail(SystemType::Network))?;

		self.api.invoke_hook(&Tag::rsa("tick"), || ())?;
		self.world.tick()?;
		ChunkModule::tick(self).wrap_err(SystemFail(SystemType::Chunk))?;
		EntityModule::tick(self).wrap_err(SystemFail(SystemType::Entity))?;

		// Send
		NetworkModule::tick(self).wrap_err(SystemFail(SystemType::Network))?;
		Ok(())
	}

	pub fn reload(&mut self, api: &Api) {
		self.chunk.reload(api);
		self.player.reload(api);
		self.entity.reload(api);
	}
}

#[derive(Debug)]
pub struct SystemFail(SystemType);

impl Display for SystemFail {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)?;
		f.write_str(" systems failure.")
	}
}

#[derive(Debug)]
pub struct CarrierUnavailable;

impl Display for CarrierUnavailable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Carrier is unavailable, Force reloading instance.")
	}
}

#[derive(Debug)]
pub enum SystemType {
	Entity,
	Chunk,
	Network,
	Player,
}
