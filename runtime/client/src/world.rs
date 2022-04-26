use rustaria::{ClientNetwork, Server, SmartError};
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::packet::{ClientPacket, ServerPacket};
use rustaria::packet::entity::ServerEntityPacket;
use rustaria::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_api::ty::{RawId, Tag};
use rustaria_util::error::{ContextCompat, Report};
use rustaria_util::error::Result;
use rustaria_util::Uuid;
use rustariac_backend::ty::Camera;

use crate::ControllerHandler;
use crate::internal::chunk::ChunkHandler;
use crate::internal::entity::EntityHandler;

/// Only exists in Client if it has joined a server.
/// This keeps all of the logic that is present when you are in a world.
pub(crate) struct ClientWorld {
	pub networking: ClientNetwork,
	pub chunk: ChunkHandler,

	pub player_entity: Option<RawId>,
	pub player: Option<Uuid>,
	pub entity: EntityHandler,

	pub integrated: Option<Box<Server>>,
}

impl ClientWorld {
	pub fn tick(
		&mut self,
		camera: &mut rustariac_backend::ty::Camera,
		controller: &mut ControllerHandler,
	) -> Result<()> {
		if let Some(player) = &self.player {
			let physics = self
				.entity
				.physics
				.get_mut(player)
				.wrap_err("Player entity velocity does not exist")?;
			controller.tick(physics);
		}

		self.chunk.tick(camera, &mut self.networking)?;
		self.entity.tick(camera, &self.chunk)?;

		if let Some(player) = &self.player {
			let position = self
				.entity
				.position
				.get(player)
				.wrap_err("Player entity does not exist")?
				.position;
			self.networking
				.send(ClientPacket::Player(ClientPlayerPacket::SetPos(position)))?;

			camera.position = position.to_array();
		}

		if let Some(integrated) = &mut self.integrated {
			integrated.tick()?;
		}

		self.networking.poll::<Report, _>(|packet| match packet {
			ServerPacket::Chunk(packet) => self.chunk.packet(packet),
			ServerPacket::Entity(packet) => self.entity.packet(packet),
			ServerPacket::Player(packet) => match packet {
				ServerPlayerPacket::Attach { entity, pos } => {
					self.player = Some(entity);
					self.entity.packet(ServerEntityPacket::New(
						entity,
						self.player_entity
							.wrap_err(SmartError::CarrierUnavailable)?,
						pos,
					))?;
					Ok(())
				}
			},
		})?;

		Ok(())
	}

	pub fn draw(&mut self, camera: &mut Camera, delta: f32) -> Result<()> {
		if let Some(player) = &self.player {
			//self.entity
			//	.physics
			//	.get_mut(player)
			//	.wrap_err("Player entity velocity does not exist")?
			//	.velocity = Vector2D::from(camera.velocity);
			//
			camera.position = self
				.entity
				.position
				.get(player)
				.wrap_err("Player entity position does not exist")?
				.position
				.to_array();
		}
		self.chunk.draw(camera);
		self.entity.draw(camera, delta)?;

		Ok(())
	}
}

impl Reloadable for ClientWorld {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		let access = carrier.lock();
		self.player_entity = access
			.get_registry::<EntityPrototype>()
			.id_from_tag(&Tag::new("rustaria:player").unwrap());

		self.chunk.reload(api, carrier);
		self.entity.reload(api, carrier);
		if let Some(server) = &mut self.integrated {
			server.reload(api, carrier);
		}
	}
}
