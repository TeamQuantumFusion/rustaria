use rsa_core::api::carrier::Carrier;
use rsa_core::api::{Api, Reloadable};
use rsa_core::error::Result;
use rsa_network::Token;
use rsa_network::tunnel::MapTunnel;

use crate::entity::packet::ClientEntityPacket;
use crate::entity::systems::server_network::ServerNetworkECSystem;
use crate::Server;

pub struct EntityModule {
	carrier: Option<Carrier>,
	network: ServerNetworkECSystem,
}

impl EntityModule {
	pub fn new() -> EntityModule {
		EntityModule {
			carrier: None,
			network: ServerNetworkECSystem::default(),
		}
	}

	#[macro_module::module(server.entity)]
	pub fn tick(this: &mut EntityModule, server: &mut Server) -> Result<()> {
		this.network.tick(&mut server.world.entities, &mut server.network.map())?;
		Ok(())
	}

	#[macro_module::module(server.entity)]
	pub fn packet(
		this: &mut EntityModule,
		server: &mut Server,
		token: Token,
		packet: ClientEntityPacket,
	) -> Result<()> {
		this.network
			.packet(&server.player, &mut server.world.entities, &token, packet)
	}

	pub fn reload(&mut self, api: &Api) {
		self.carrier = Some(api.get_carrier());
	}
}
