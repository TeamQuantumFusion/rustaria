#![allow(clippy::new_without_default)]

pub mod api;
#[macro_use]
pub mod network;
mod player;

use apollo::LuaScope;
use rsa_core::{
	api::Core,
	debug::DummyRenderer,
	err::{ext::AuditExt, Result},
	log::info,
};
use rsa_core::log::trace;
use rsa_network::{server::ServerNetwork};
use rsa_world::{ World};

use crate::{network::ServerBoundPacket, player::PlayerSystem, api::RustariaAPI};

pub struct Rustaria {
	network: ServerNetwork<Rustaria>,
	player: PlayerSystem,
	world: World,
}

impl Rustaria {
	pub fn new(
		rpc: &RustariaAPI,
		network: ServerNetwork<Rustaria>,
		world: World,
	) -> Result<Rustaria> {
		info!("Launching integrated server.");
		Ok(Rustaria {
			network,
			player: PlayerSystem::new(&rpc.world)?,
			world,
		})
	}

	pub fn tick(&mut self, core: &Core, rpc: &RustariaAPI) -> Result<()> {
		for (token, packet) in self.network.poll() {
			match packet {
				ServerBoundPacket::Player(packet) => {
					self.player
						.packet(&rpc.world, token, packet, &mut self.world);
				}
				ServerBoundPacket::World(packet) => {
					self.world.packet(
						&rpc.world,
						token,
						packet,
						&mut self.network.sender().map(),
					)?;
				}
			}
		}

		self.world
			.tick(core, &rpc.world, &mut DummyRenderer)
			.wrap_err("Ticking world")?;
		self.player
			.tick(&mut self.network.sender().map(), &self.world)
			.wrap_err("Ticking player system.")?;
		Ok(())
	}
}
