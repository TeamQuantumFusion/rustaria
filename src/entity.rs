pub mod hitbox;
pub mod pos;
pub mod velocity;

use std::collections::{HashMap, HashSet};

use eyre::{ContextCompat, Result};
use pos::PositionComp;

use rustaria_api::ty::RawId;
use rustaria_api::{Carrier, Reloadable};
use rustaria_util::math::{Vector2D, WorldSpace};

use rustaria_util::{info, Uuid};
use velocity::VelocityComp;

use crate::api::prototype::entity::EntityPrototype;
use crate::chunk::ChunkContainer;
use crate::entity::hitbox::HitboxComp;
use crate::SmartError;

#[derive(Default)]
pub struct EntityContainer {
	carrier: Option<Carrier>,
	pub entities: HashMap<Uuid, RawId>,
	pub position: HashMap<Uuid, PositionComp>,
	pub velocity: HashMap<Uuid, VelocityComp>,
	pub hitbox: HashMap<Uuid, HitboxComp>,
	pub dead: HashSet<Uuid>,
}

impl EntityContainer {
	pub fn spawn(&mut self, id: RawId, pos: Vector2D<f32, WorldSpace>) -> Result<Uuid> {
		let carrier = self
			.carrier
			.as_ref()
			.wrap_err(SmartError::CarrierUnavailable)?
			.lock();

		// Get uuid and handle conflicts by re-rolling until you find a spot.
		let mut uuid = Uuid::new_v4();
		while self.entities.contains_key(&uuid) {
			uuid = Uuid::new_v4();
		}
		self.entities.insert(uuid, id);

		// Get prototype
		let prototype = carrier
			.get_registry::<EntityPrototype>()
			.prototype_from_id(id)
			.wrap_err("Could not find entity")?;

		// Add components
		self.position.insert(uuid, PositionComp { position: pos });

		if let Some(hitbox) = &prototype.hitbox {
			self.hitbox.insert(uuid, hitbox.clone());
		}

		if let Some(velocity) = &prototype.velocity {
			self.velocity.insert(uuid, velocity.clone());
		}

		Ok(uuid)
	}

	pub fn kill(&mut self, id: Uuid) {
		self.dead.insert(id);
	}

	pub fn tick(&mut self, chunks: &ChunkContainer) {
		for id in self.dead.drain() {
			self.entities.remove(&id);
			self.position.remove(&id);
			self.velocity.remove(&id);
			self.hitbox.remove(&id);
		}

		for (id, velocity) in &mut self.velocity {
			// required
			let position = self.position.get_mut(id).unwrap();
			position.position += velocity.velocity;
		}

		for (id, hitbox) in &mut self.hitbox {
			let position = self.position.get_mut(id).unwrap();
			position.position = hitbox::tile_collision(position.position, hitbox, chunks);
		}
	}
}

impl Reloadable for EntityContainer {
	fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
	}
}
