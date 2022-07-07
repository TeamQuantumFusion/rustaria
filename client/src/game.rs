use glfw::WindowEvent;
use rsa_client_core::{debug::Debug, frontend::Frontend, timing::Timing, ty::Viewport};
use rsa_client_graphics::world::WorldRenderer;
use rsa_client_player::PlayerSystem;
use rsa_core::{
	api::Core,
	err::{ext::AuditExt, Result},
};
use rsa_network::{client::ClientNetwork, new_networking};
use rsa_player::packet::ServerBoundPlayerPacket;
use rsa_world::{chunk::storage::ChunkStorage, World};
use rustaria::{network::ClientBoundPacket, Rustaria};

use crate::{game::world::ClientWorld, ClientRPC};

mod world;

/// This exists when a client has joined a world.
pub struct ClientGame {
	integrated: Option<Rustaria>,

	pub network: ClientNetwork<Rustaria>,
	pub player: PlayerSystem,
	pub world: ClientWorld,
	pub renderer: WorldRenderer,
}

impl ClientGame {
	pub fn new_integrated(
		frontend: &Frontend,
		rpc: &ClientRPC,
		world: World,
	) -> Result<ClientGame> {
		let (network, server_network) = new_networking();
		// Send join packet
		network.send(ServerBoundPlayerPacket::Join().into())?;

		Ok(ClientGame {
			network,
			player: PlayerSystem::new(&rpc.world)?,
			world: ClientWorld::new(World::new(ChunkStorage::new(
				world.chunks.width(),
				world.chunks.height(),
			))?),
			renderer: WorldRenderer::new(frontend)?,
			integrated: Some(
				Rustaria::new(rpc, server_network, world).wrap_err("Failed to start server")?,
			),
		})
	}

	pub fn event(&mut self, frontend: &Frontend, event: WindowEvent) {
		self.player.event(event, frontend);
	}

	pub fn get_viewport(&mut self) -> Option<Viewport> { Some(self.player.get_viewport()) }

	pub fn tick(
		&mut self,
		frontend: &Frontend,
		core: &Core,
		rpc: &ClientRPC,
		viewport: &Viewport,
		debug: &mut Debug,
	) -> Result<()> {
		if let Some(server) = &mut self.integrated {
			server
				.tick(core, rpc)
				.wrap_err("Ticking integrated server")?;
		}
		for packet in self.network.poll() {
			match packet {
				ClientBoundPacket::Player(packet) => {
					self.player
						.packet(core, &rpc.world, packet, &mut self.world)?;
				}
				ClientBoundPacket::World(packet) => {
					self.world.packet(&rpc.world, packet)?;
				}
			}
		}
		let mut network = self.network.sender();
		self.player.tick(
			core,
			&rpc.world,
			viewport,
			&mut network.map(),
			&mut self.world,
		)?;
		self.world
			.tick_client(core, rpc, &self.player, &mut network.map(), debug)?;
		self.renderer.tick(frontend, &self.world)?;
		self.world.chunks.reset_dirty();
		Ok(())
	}

	pub fn reset(&mut self) {
		self.world.chunks.reset();
		self.renderer.reload();
	}
}
