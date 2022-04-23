use crate::RenderingHandler;
use eyre::{ContextCompat, Result};
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::chunk::ChunkStorage;
use rustaria::entity::world::EntityWorld;
use rustaria::packet::entity::ServerEntityPacket;
use rustaria::SmartError;
use rustaria_api::{Api, Carrier, Reloadable};
use rustariac_backend::ty::Camera;
use rustariac_rendering::entity_drawer::WorldEntityDrawer;

pub(crate) struct EntityHandler {
	carrier: Option<Carrier>,
	world: EntityWorld,
	drawer: WorldEntityDrawer,
}

impl EntityHandler {
	pub fn new(rendering: &RenderingHandler) -> EntityHandler {
		EntityHandler {
			carrier: None,
			world: Default::default(),
			drawer: WorldEntityDrawer::new(&rendering.backend),
		}
	}

	pub fn packet(&mut self, packet: ServerEntityPacket) -> Result<()> {
		let carrier = self
			.carrier
			.as_ref()
			.wrap_err(SmartError::CarrierUnavailable)?;

		let access = carrier.lock();
		let registry = access.get_registry::<EntityPrototype>();
		match packet {
			ServerEntityPacket::New(uuid, id, pos) => {
				let prototype = registry
					.prototype_from_id(id)
					.wrap_err("Server Entity prototype does not exist.")?;
				self.world.insert(uuid, id, pos, prototype);
			}
			ServerEntityPacket::Kill(uuid) => {
				self.world.remove(uuid);
			}
			ServerEntityPacket::SetPos(uuid, pos) => {
				self.world
					.position
					.get_mut(&uuid)
					.expect("entity re-stabilizing not implemented.")
					.position = pos;
			}
		}

		Ok(())
	}
	pub fn draw(&mut self, camera: &Camera, delta: f32) -> Result<()> {
		self.drawer.draw(camera, &self.world, delta)?;

		Ok(())
	}

	pub fn tick(&mut self, camera: &Camera, chunks: &ChunkStorage) -> Result<()> {
		self.world.tick(chunks)?;
		self.drawer.tick(camera, &self.world)?;

		Ok(())
	}
}

impl Reloadable for EntityHandler {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
		self.drawer.reload(api, carrier);
	}
}
