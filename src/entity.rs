use std::sync::Arc;

use eyre::{ContextCompat, Result};
pub use legion::*;
use legion::{Entity, Resources, Schedule};
use rayon::ThreadPool;
use serde::Deserialize;

use rustaria_api::ty::{Prototype, RawId};
use rustaria_api::{Carrier, Reloadable};
use rustaria_util::ty::pos::Pos;

use crate::api::prototype::entity::EntityPrototype;
use crate::SmartError;

/// To prevent conflicts with rustaria::World and legion::World.
type Universe = legion::World;

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

#[legion::system(for_each)]
pub fn update_positions(pos: &mut PositionComp, vel: &VelocityComp) {
	pos.position += vel.velocity;
}

pub struct EntityContainer {
	carrier: Option<Carrier>,
	universe: Universe,
	schedule: Schedule,
	resources: Resources,
	thread_pool: Arc<ThreadPool>,
}

impl EntityContainer {
	pub fn new(thread_pool: Arc<ThreadPool>) -> EntityContainer {
		EntityContainer {
			carrier: None,
			universe: Universe::default(),
			resources: Resources::default(),
			schedule: Schedule::builder()
				.add_system(update_positions_system())
				.build(),
			thread_pool,
		}
	}

	pub fn spawn(&mut self, id: RawId, position: Pos) -> Result<Entity> {
		let carrier = self
			.carrier
			.as_ref()
			.wrap_err(SmartError::CarrierUnavailable)?;
		// Create entity and get its entry to add dynamic components.
		let entity = self.universe.push((IdComp(id), PositionComp { position }));
		let mut entry = self.universe.entry(entity).unwrap();

		// Get instance, get prototype and add all of the needed components.
		let instance = carrier.lock();
		let prototype = instance
			.get_registry::<EntityPrototype>()
			.prototype_from_id(id)
			.wrap_err("Could not find entity")?;
		if let Some(velocity) = &prototype.velocity {
			entry.add_component(velocity.create(id));
		}

		Ok(entity)
	}

	pub fn tick(&mut self) {
		self.schedule.execute_in_thread_pool(
			&mut self.universe,
			&mut self.resources,
			&self.thread_pool,
		);
	}
}

impl Reloadable for EntityContainer {
	fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
	}
}
