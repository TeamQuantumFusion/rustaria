use eyre::{Result, WrapErr};
use glfw::WindowEvent;
use glium::Frame;
use rustaria::{
	network::{new_networking, packet::ClientBoundPacket, ClientNetwork},
	player::ServerBoundPlayerPacket,
	world::{chunk::storage::ChunkStorage, World},
	Server,
};

use crate::{
	game::world::ClientWorld,
	render::{ty::viewport::Viewport, world::WorldRenderer},
	ClientApi, Debug, Frontend, PlayerSystem, Timing,
};

mod network;
pub mod player;
mod world;

/// This exists when a client has joined a world.
pub struct ClientGame {
	integrated: Option<Server>,

	network: ClientNetwork,
	player: PlayerSystem,
	world: ClientWorld,

	pub renderer: WorldRenderer,
}

impl ClientGame {
	pub fn new_integrated(
		frontend: &Frontend,
		api: &ClientApi,
		world: World,
	) -> Result<ClientGame> {
		let (network, server_network) = new_networking();
		// Send join packet
		network.send(ServerBoundPlayerPacket::Join())?;

		Ok(ClientGame {
			network,
			player: PlayerSystem::new(api)?,
			world: ClientWorld::new(World::new(
				api,
				ChunkStorage::new(world.chunks.width(), world.chunks.height()),
			)?),
			renderer: WorldRenderer::new(frontend, api)?,
			integrated: Some(
				Server::new(api, server_network, world).wrap_err("Failed to start server")?,
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
		api: &ClientApi,
		viewport: &Viewport,
		debug: &mut Debug,
	) -> Result<()> {
		if let Some(server) = &mut self.integrated {
			server.tick(api).wrap_err("Ticking integrated server")?;
		}
		for packet in self.network.poll() {
			match packet {
				ClientBoundPacket::Player(packet) => {
					self.player.packet(api, packet, &mut self.world)?;
				}
				ClientBoundPacket::World(packet) => {
					self.world.packet(api, packet, debug)?;
				}
			}
		}
		self.player
			.tick(api, viewport, &mut self.network, &mut self.world)?;
		self.world
			.tick_client(api, &self.player, &mut self.network, debug)?;
		self.renderer
			.tick(frontend, &self.player, &self.world, debug)?;
		self.world.chunks.reset_dirty();
		Ok(())
	}

	pub fn draw(
		&mut self,
		api: &ClientApi,
		frontend: &Frontend,
		frame: &mut Frame,
		viewport: &Viewport,
		debug: &mut Debug,
		timing: &Timing,
	) -> Result<()> {
		self.renderer.draw(
			api,
			frontend,
			&self.player,
			&self.world,
			frame,
			viewport,
			debug,
			timing,
		)?;
		Ok(())
	}

	pub fn reset(&mut self) {
		self.world.chunks.reset();
		self.renderer.reload();
	}
}
