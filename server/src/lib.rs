#![allow(clippy::new_without_default)]

pub mod rpc;
#[macro_use]
pub mod network;
mod player;

use rsa_core::{
	api::Core,
	debug::DummyRenderer,
	err::{ext::AuditExt, Result},
	log::info,
};
use rsa_network::{server::ServerNetwork};
use rsa_world::{ World};

use crate::{network::ServerBoundPacket, player::PlayerSystem, rpc::ServerRPC};

pub struct Rustaria {
	network: ServerNetwork<Rustaria>,
	player: PlayerSystem,
	world: World,
}

impl Rustaria {
	pub fn new(
		rpc: &ServerRPC,
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

	pub fn tick(&mut self, core: &Core, rpc: &ServerRPC) -> Result<()> {
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
