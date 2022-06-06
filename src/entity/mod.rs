pub mod component;
pub mod systems;

pub mod packet;
pub mod prototype;

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use hecs::{Component, ComponentError, DynamicBundle, Entity, Query, QueryBorrow, QueryMut, Ref, RefMut};

use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::{Prototype, RawId};
use rsa_core::error::Result;
use crate::chunk::ChunkSystem;

use crate::entity::component::pos::PositionComp;
use crate::entity::prototype::EntityPrototype;
use crate::entity::systems::collision::CollisionECSystem;
use crate::entity::systems::gravity::GravityECSystem;
use crate::entity::systems::movement::MovementECSystem;

pub struct EntityStorage {
	data: hecs::World,
}

impl EntityStorage {
	pub fn push(&mut self, components: impl DynamicBundle) -> Entity {
		self.data.spawn(components)
	}

	pub fn insert(&mut self, entity: Entity, components: impl DynamicBundle) {
		self.data.spawn_at(entity, components);
	}

	pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<'_, T>, ComponentError> {
		self.data.get(entity)
	}

	pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Result<RefMut<'_, T>, ComponentError> {
		self.data.get_mut(entity)
	}

	pub fn query<Q: Query>(&self) -> QueryBorrow<'_, Q>  {
		self.data.query()
	}

	pub fn query_mut<Q: Query>(&mut self) -> QueryMut<'_, Q> {
		self.data.query_mut()
	}

	pub fn kill(&mut self, entity: Entity) {
		let _result = self.data.take(entity);
	}
}

pub struct EntitySystem {
	storage: EntityStorage,
	gravity_system: GravityECSystem,
	movement_system: MovementECSystem,
	collision_system: CollisionECSystem,

	// Keep out of reach of children
	dead: HashSet<Entity>,
}

impl EntitySystem {
	pub fn new() -> EntitySystem {
		EntitySystem  {
			storage: EntityStorage {
				data: Default::default()
			},
			gravity_system: Default::default(),
			movement_system: Default::default(),
			collision_system: Default::default(),
			dead: Default::default()
		}
	}
	
	pub fn spawn(
		&mut self,
		position: Vector2D<f32, WorldSpace>,
		id: RawId,
		prototype: &EntityPrototype,
	) -> Entity {
		let mut builder = prototype.create(id);
		builder.add(PositionComp { position });
		self.storage.push(builder.build())
	}

	pub fn spawn_at(
		&mut self,
		entity: Entity,
		position: Vector2D<f32, WorldSpace>,
		id: RawId,
		prototype: &EntityPrototype,
	) {
		let mut builder = prototype.create(id);
		builder.add(PositionComp { position });
		self.storage.insert(entity, builder.build());
	}

	pub fn tick(&mut self, chunks: &ChunkSystem) -> Result<()> {
		self.gravity_system.tick(&mut self.storage);
		self.movement_system.tick(&mut self.storage);
		self.collision_system.tick(&mut self.storage, chunks);
		Ok(())
	}
}

impl Deref for EntitySystem  {
	type Target = EntityStorage;

	fn deref(&self) -> &Self::Target {
		&self.storage
	}
}

impl DerefMut for EntitySystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.storage
	}
}
