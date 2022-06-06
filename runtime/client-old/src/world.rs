use rsa_core::api::{Api, Reloadable};
use rsa_core::error::ContextCompat;
use rsa_core::ty::{RawId, Tag};
use rsa_network::client::ClientTickData;
use rsac_backend::ty::Camera;
use rustaria::entity::prototype::EntityPrototype;
use rustaria::packet::entity::ServerEntityPacket;
use rustaria::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use rustaria::packet::{ClientPacket, ServerPacket};
use rustaria::player::Player;
use rustaria::{ClientNetwork, Server, RichError};
use rsa_core::error::Result;

use crate::module::chunk::ChunkHandler;
use crate::module::entity::EntityHandler;
use crate::ControllerHandler;

/// Only exists in Client if it has joined a server.
/// This keeps all of the logic that is present when you are in a world.
pub(crate) struct ClientWorld {
	pub networking: ClientNetwork,
	pub chunk: ChunkHandler,

	pub player_entity_id: Option<RawId>,
	pub player: Player,
	pub entity: EntityHandler,

	pub integrated: Option<Box<Server>>,
}

impl ClientWorld {
	pub fn tick(
		&mut self,
		camera: &mut Camera,
		controller: &mut ControllerHandler,
	) -> Result<()> {
		if let Some(player) = &self.player.entity {
			let hitbox = self
				.entity
				.hitbox
				.get(player)
				.wrap_err("Player entity velocity does not exist")?
				.touches_ground;
			let physics = self
				.entity
				.physics
				.get_mut(player)
				.wrap_err("Player entity velocity does not exist")?;

			controller.apply(physics, hitbox, &self.player);
		}

		self.chunk.tick(camera, &mut self.networking)?;
		self.entity.tick(camera, &self.chunk)?;

		if let Some(player) = &self.player.entity {
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

		match self.networking.tick()? {
			ClientTickData::Received(packets) => {
				for packet in packets {
					match packet {
						ServerPacket::Chunk(packet) => self.chunk.packet(packet)?,
						ServerPacket::Entity(packet) => self.entity.packet(packet)?,
						ServerPacket::Player(packet) => match packet {
							ServerPlayerPacket::Attach { entity, pos } => {
								self.player.entity = Some(entity);
								self.entity.packet(ServerEntityPacket::New(
									entity,
									self.player_entity_id
										.wrap_err(RichError::CarrierUnavailable)?,
									pos,
								))?;
							}
						},
					}
				}
			}
			ClientTickData::Disconnected => {
				todo!("Disconnecting sequence")
			}
		}

		Ok(())
	}

	pub fn draw(&mut self, camera: &mut Camera, delta: f32) -> Result<()> {
		if let Some(player) = &self.player.entity {
			let pos = self
				.entity
				.drawer
				.get_entity_pos(player, &self.entity.universe, delta);

			let mut pos = pos.to_array();
			if let Some(hitbox) = self.entity.hitbox.get(player) {
				pos[0] += hitbox.hitbox.min_x() + (hitbox.hitbox.width() / 2.0);
				pos[1] += hitbox.hitbox.min_y() + (hitbox.hitbox.height() / 2.0);
			}

			camera.position = pos;
		}
		self.chunk.draw(camera);
		self.entity.draw(camera, delta)?;

		Ok(())
	}
}

impl Reloadable for ClientWorld {
	fn reload(&mut self, api: &Api) {
		self.player_entity_id = Some(api.get_carrier().get::<EntityPrototype>()
			.id_from_tag(&Tag::rsa("player")).unwrap());

		self.chunk.reload(api);
		self.entity.reload(api);
		if let Some(server) = &mut self.integrated {
			server.reload(api);
		}
	}
}
