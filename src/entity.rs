use std::collections::HashMap;

use eyre::{ContextCompat, Result};
use serde::Deserialize;

use rustaria_api::ty::{Prototype, RawId};
use rustaria_api::{Carrier, Reloadable};
use rustaria_util::ty::pos::Pos;
use rustaria_util::Uuid;

use crate::api::prototype::entity::EntityPrototype;
use crate::SmartError;

#[derive(Clone, Debug, Deserialize)]
pub struct PositionComp {
	pub position: Pos,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IdComp(pub RawId);

#[derive(Clone, Debug, Deserialize)]
pub struct VelocityComp {
	pub velocity: Pos,
}

impl Default for VelocityComp {
	fn default() -> Self {
		VelocityComp {
			velocity: Pos { x: 0.0, y: 0.0 },
		}
	}
}

#[derive(Default)]
pub struct EntityWorld {
	carrier: Option<Carrier>,
	pub entities: HashMap<Uuid, RawId>,
	pub position: HashMap<Uuid, PositionComp>,
	pub velocity: HashMap<Uuid, VelocityComp>,
}

impl EntityWorld {
	pub fn spawn(&mut self, id: RawId, pos: Pos) -> Result<Uuid> {
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

		if let Some(velocity) = &prototype.velocity {
			self.velocity.insert(uuid, velocity.create(id));
		}

		Ok(uuid)
	}

	pub fn kill(&mut self, id: Uuid) {
		self.entities.remove(&id);
		self.position.remove(&id);
		self.velocity.remove(&id);
	}

	pub fn tick(&mut self) {
		for (id, velocity) in &mut self.velocity {
			if let Some(pos) = self.position.get_mut(id) {
				pos.position += velocity.velocity;
			}
		}
	}
}

impl Reloadable for EntityWorld {
	fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
	}
}
