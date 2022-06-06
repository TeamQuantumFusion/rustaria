use rsa_core::error::Result;

use crate::chunk::ChunkSystem;
use crate::entity::EntitySystem;

pub struct World {
	pub entities: EntitySystem,
	pub chunks: ChunkSystem,
}

impl World {
	pub fn new() -> World {
		World {
			entities: EntitySystem::new(),
			chunks: ChunkSystem::new()
		}
	}

	pub fn tick(&mut self) -> Result<()> {
		self.entities.tick(&self.chunks)?;
		// nothing to tick on chunks yet

		Ok(())
	}
}
