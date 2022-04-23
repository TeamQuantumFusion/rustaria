use crate::internal::chunk::ChunkHandler;
use crate::internal::entity::EntityHandler;
use crate::internal::rendering::RenderingHandler;
use crate::Client;
use eyre::Report;
use rustaria::packet::ServerPacket;
use rustaria::{ClientNetwork, Server};
use rustaria_api::{Api, Carrier, Reloadable};
use rustariac_backend::ty::Camera;

/// Only exists in Client if it has joined a server.
/// This keeps all of the logic that is present when you are in a world.
pub(crate) struct ClientWorld {
	pub networking: ClientNetwork,
	pub chunk: ChunkHandler,
	pub entity: EntityHandler,
	pub integrated: Option<Box<Server>>,
}

impl ClientWorld {
	pub fn tick(&mut self, camera: &Camera) -> eyre::Result<()> {
		self.chunk.tick(camera, &mut self.networking)?;
		self.entity.tick(camera, &self.chunk)?;
		if let Some(integrated) = &mut self.integrated {
			integrated.tick()?;
		}

		self.networking.poll::<Report, _>(|packet| match packet {
			ServerPacket::Chunk(packet) => self.chunk.packet(packet),
			ServerPacket::Entity(packet) => self.entity.packet(packet),
		})?;

		Ok(())
	}

	pub fn draw(&mut self, camera: &Camera, delta: f32) -> eyre::Result<()> {
		self.chunk.draw(camera);
		self.entity.draw(camera, delta)?;

		Ok(())
	}
}

impl Reloadable for ClientWorld {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.chunk.reload(api, carrier);
		self.entity.reload(api, carrier);
		if let Some(server) = &mut self.integrated {
			server.reload(api, carrier);
		}
	}
}
