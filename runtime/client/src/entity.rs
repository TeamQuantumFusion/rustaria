use eyre::Result;
use rustaria::chunk::ChunkContainer;
use rustaria::entity::EntityContainer;
use rustaria::network::packet::entity::ServerEntityPacket;
use rustaria_api::{Api, Carrier, Reloadable};
use rustariac_backend::ty::Camera;
use rustariac_backend::ClientBackend;
use rustariac_rendering::entity_drawer::WorldEntityDrawer;

pub(crate) struct EntityHandler {
	container: EntityContainer,
	drawer: WorldEntityDrawer,
}

impl EntityHandler {
	pub fn new(backend: &ClientBackend) -> EntityHandler {
		EntityHandler {
			container: EntityContainer::default(),
			drawer: WorldEntityDrawer::new(backend),
		}
	}

	pub fn packet(&mut self, packet: ServerEntityPacket) -> Result<()> {
		match packet {
			ServerEntityPacket::New(id, pos) => {
				self.container.spawn(id, pos)?;
			}
		}

		Ok(())
	}
	pub fn draw(&mut self, camera: &Camera, delta: f32) -> Result<()> {
		self.drawer.draw(camera, &self.container, delta)?;

		Ok(())
	}

	pub fn tick(&mut self, camera: &Camera, chunks: &ChunkContainer) -> Result<()> {
		self.drawer.tick(camera, &self.container)?;
		self.container.tick(chunks);

		Ok(())
	}
}

impl Reloadable for EntityHandler {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.container.reload(api, carrier);
		self.drawer.reload(api, carrier);
	}
}
