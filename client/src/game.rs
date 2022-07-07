use glfw::WindowEvent;
use rsa_core::api::Core;
use rsa_core::err::ext::AuditExt;
use rsa_core::err::Result;
use rsa_network::client::ClientNetwork;
use rsa_network::new_networking;
use rsa_player::packet::ServerBoundPlayerPacket;
use rsa_world::chunk::storage::ChunkStorage;
use rsa_world::World;
use rsaclient_core::debug::Debug;
use rsaclient_core::frontend::Frontend;
use rsaclient_core::timing::Timing;
use rsaclient_core::ty::Viewport;
use rsaclient_graphics::world::WorldRenderer;
use rsaclient_player::PlayerSystem;
use rustaria::network::ClientBoundPacket;
use rustaria::Rustaria;
use crate::ClientRPC;
use crate::game::world::ClientWorld;

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
			world: ClientWorld::new(World::new(
				ChunkStorage::new(world.chunks.width(), world.chunks.height()),
			)?),
			renderer: WorldRenderer::new(frontend)?,
			integrated: Some(
				Rustaria::new(rpc, server_network, world).wrap_err("Failed to start server")?,
			),
		})
	}

	pub fn event(&mut self, frontend: &Frontend, event: WindowEvent) {
		self.player.event(event, frontend);
	}

	pub fn get_viewport(&mut self) -> Option<Viewport> {
		Some(self.player.get_viewport())
	}

	pub fn tick(
		&mut self,
		frontend: &Frontend,
		core: &Core,
		rpc: &ClientRPC,
		viewport: &Viewport,
		debug: &mut Debug,
	) -> Result<()> {
		if let Some(server) = &mut self.integrated {
			server.tick(core, rpc).wrap_err("Ticking integrated server")?;
		}
		for packet in self.network.poll() {
			match packet {
				ClientBoundPacket::Player(packet) => {
					self.player.packet(core, &rpc.world, packet, &mut self.world)?;
				}
				ClientBoundPacket::World(packet) => {
					self.world.packet(&rpc.world, packet)?;
				}
			}
		}
		let mut network = self.network.sender();
		self.player
			.tick(core, &rpc.world, viewport, &mut network.map(), &mut self.world)?;
		self.world
			.tick_client(core, rpc, &self.player, &mut network.map(), debug)?;
		self.renderer
			.tick(frontend,  &self.world)?;
		self.world.chunks.reset_dirty();
		Ok(())
	}

	pub fn reset(&mut self) {
		self.world.chunks.reset();
		self.renderer.reload();
	}
}
