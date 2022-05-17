use std::collections::{HashMap, HashSet};

use rustaria_api::ty::RawId;
use rustaria_common::error::Result;
use rustaria_common::logging::info;
use rustaria_common::math::{Vector2D, WorldSpace};
use rustaria_common::settings::UPS;
use rustaria_common::Uuid;

use crate::api::prototype::entity::EntityPrototype;
use crate::chunk::ChunkStorage;
use crate::entity::component::gravity::GravityComp;
use crate::entity::component::hitbox;
use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::pos::PositionComp;
use crate::entity::component::velocity::PhysicsComp;

/// An entity world holds all of the entities and simulates them.
#[derive(Default)]
pub struct EntityWorld {
	// Components
	pub entities: HashMap<Uuid, RawId>,
	pub position: HashMap<Uuid, PositionComp>,
	pub physics: HashMap<Uuid, PhysicsComp>,
	pub hitbox: HashMap<Uuid, HitboxComp>,
	pub gravity: HashMap<Uuid, GravityComp>,

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
			self.physics.insert(uuid, velocity.clone());
		}
		if let Some(gravity) = &prototype.gravity {
			self.gravity.insert(uuid, gravity.clone());
		}
	}

	/// Remove the entity
	pub fn remove(&mut self, uuid: Uuid) {
		self.entities.remove(&uuid);
		self.position.remove(&uuid);
		self.physics.remove(&uuid);
		self.gravity.remove(&uuid);
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
			self.physics.remove(&uuid);
			self.gravity.remove(&uuid);
			self.hitbox.remove(&uuid);
		}

		for (id, gravity) in &self.gravity {
			if let Some(physics) = self.physics.get_mut(id) {
				physics.velocity.y -= 1.6 / UPS as f32;
			}
		}

		for (id, physics) in &mut self.physics {
			physics.velocity.y = physics.velocity.y.max(-40.0 / UPS as f32);

			let position = self.position.get_mut(id).unwrap();
			if let Some(hitbox) = self.hitbox.get_mut(id) {
				hitbox::tile_collision(position.position, physics, hitbox, chunks);
			}

			position.position += physics.velocity;
		}

		Ok(())
	}
}
