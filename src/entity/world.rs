use crate::api::prototype::entity::EntityPrototype;
use crate::chunk::ChunkStorage;
use crate::entity::component::hitbox;
use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::pos::PositionComp;
use crate::entity::component::velocity::VelocityComp;
use eyre::Result;
use rustaria_api::ty::RawId;
use rustaria_util::math::{Vector2D, WorldSpace};
use rustaria_util::Uuid;
use std::collections::{HashMap, HashSet};

/// An entity world holds all of the entities and simulates them.
#[derive(Default)]
pub struct EntityWorld {
	// Components
	pub entities: HashMap<Uuid, RawId>,
	pub position: HashMap<Uuid, PositionComp>,
	pub velocity: HashMap<Uuid, VelocityComp>,
	pub hitbox: HashMap<Uuid, HitboxComp>,

	pub dead: HashSet<Uuid>,
}

impl EntityWorld {
	pub fn insert(
		&mut self,
		uuid: Uuid,
		id: RawId,
		position: Vector2D<f32, WorldSpace>,
		prototype: &EntityPrototype,
	) {
		self.entities.insert(uuid, id);

		// Add components
		self.position.insert(uuid, PositionComp { position });

		if let Some(hitbox) = &prototype.hitbox {
			self.hitbox.insert(uuid, hitbox.clone());
		}

		if let Some(velocity) = &prototype.velocity {
			self.velocity.insert(uuid, velocity.clone());
		}
	}

	/// Remove the entity
	pub fn remove(&mut self, uuid: Uuid) {
		self.entities.remove(&uuid);
		self.position.remove(&uuid);
		self.velocity.remove(&uuid);
		self.hitbox.remove(&uuid);
	}

	/// Notify about killing the entity.
	pub fn kill(&mut self, uuid: Uuid) {
		self.dead.insert(uuid);
	}

	pub fn tick(&mut self, chunks: &ChunkStorage) -> Result<()> {
		for uuid in self.dead.drain() {
			self.entities.remove(&uuid);
			self.position.remove(&uuid);
			self.velocity.remove(&uuid);
			self.hitbox.remove(&uuid);
		}

		for (id, velocity) in &mut self.velocity {
			let position = self.position.get_mut(id).unwrap();
			position.position += velocity.velocity;
		}

		for (id, hitbox) in &mut self.hitbox {
			let position = self.position.get_mut(id).unwrap();
			position.position = hitbox::tile_collision(position.position, hitbox, chunks);
		}

		Ok(())
	}
}
