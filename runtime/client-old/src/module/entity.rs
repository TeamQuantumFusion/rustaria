use std::ops::{Deref, DerefMut};

use rustaria::entity::prototype::EntityPrototype;
use rustaria::chunk::ChunkStorage;
use rustaria::entity::world::EntityWorld;
use rustaria::packet::entity::ServerEntityPacket;
use rustaria::RichError;
use rsa_core::api::{Api, Reloadable};
use rsa_core::api::carrier::Carrier;
use rsa_core::error::{ContextCompat, Result};
use rsac_backend::ty::Camera;
use rsac_rendering::entity_drawer::WorldEntityDrawer;
use rustaria::entity::EntitySystem;

use crate::RenderingHandler;

pub struct EntityHandler {
	carrier: Option<Carrier>,
	entity_world: EntitySystem,
	drawer: WorldEntityDrawer,
}

impl EntityHandler {
	pub fn new(rendering: &RenderingHandler) -> EntityHandler {
		EntityHandler {
			carrier: None,
			entity_world: Default::default(),
			drawer: WorldEntityDrawer::new(&rendering.backend),
		}
	}

	pub fn packet(&mut self, packet: ServerEntityPacket) -> Result<()> {
		let carrier = self
			.carrier
			.as_ref()
			.wrap_err(RichError::CarrierUnavailable)?;

		match packet {
			ServerEntityPacket::New(uuid, id, pos) => {

				self.universe.spawn(uuid, id, pos, carrier.get::<EntityPrototype>()
					.prototype_from_id(id));
			}
			ServerEntityPacket::Kill(uuid) => {
				self.universe.remove(uuid);
			}
			ServerEntityPacket::SetPos(uuid, pos) => {
				self.universe
					.position
					.get_mut(&uuid)
					.expect("entity re-stabilizing not implemented.")
					.position = pos;
			}
		}

		Ok(())
	}
	pub fn draw(&mut self, camera: &Camera, delta: f32) -> Result<()> {
		self.drawer.draw(camera, &self.universe, delta)?;

		Ok(())
	}

	pub fn tick(&mut self, camera: &Camera, chunks: &ChunkStorage) -> Result<()> {
		self.drawer.tick(camera, &self.universe)?;
		self.universe.tick(chunks)?;

		Ok(())
	}
}

impl Reloadable for EntityHandler {
	fn reload(&mut self, api: &Api) {
		self.carrier = Some(api.get_carrier());
		self.drawer.reload(api);
	}
}

impl Deref for EntityHandler {
	type Target = EntitySystem;

	fn deref(&self) -> &Self::Target {
		&self.universe
	}
}

impl DerefMut for EntityHandler {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.universe
	}
}
