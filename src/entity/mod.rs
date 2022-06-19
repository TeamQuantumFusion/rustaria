pub mod component;
pub mod systems;

pub mod packet;
pub mod prototype;

pub use hecs::{
	Component, ComponentError, DynamicBundle, Entity, Query, QueryBorrow, QueryMut, Ref, RefMut,
};
use hecs::{EntityBuilder, EntityRef, TakenEntity};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use crate::chunk::ChunkSystem;
use crate::entity::component::gravity::GravityComp;
use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::humanoid::HumanoidComp;
use crate::entity::component::physics::PhysicsComp;
use rsa_core::error::Result;
use rsa_core::logging::trace;
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::{Prototype, RawId};

use crate::entity::component::pos::PositionComp;
use crate::entity::component::prototype::PrototypeComp;
use crate::entity::prototype::EntityPrototype;
use crate::entity::systems::collision::CollisionECSystem;
use crate::entity::systems::gravity::GravityECSystem;
use crate::entity::systems::movement::MovementECSystem;
use crate::entity::systems::physics::PhysicsECSystem;

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

	pub fn get_entity(&self, entity: Entity) -> Option<EntityRef<'_>> {
		self.data.entity(entity).ok()
	}

	pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<'_, T>, ComponentError> {
		self.data.get(entity)
	}

	pub fn get_mut<T: Component>(
		&mut self,
		entity: Entity,
	) -> Result<RefMut<'_, T>, ComponentError> {
		self.data.get_mut(entity)
	}

	pub fn query<Q: Query>(&self) -> QueryBorrow<'_, Q> {
		self.data.query()
	}

	pub fn query_mut<Q: Query>(&mut self) -> QueryMut<'_, Q> {
		self.data.query_mut()
	}

	pub fn kill(&mut self, entity: Entity) -> Option<TakenEntity<'_>> {
		self.data.take(entity).ok()
	}

	pub fn clear(&mut self) {
		self.data.clear();
	}

	pub fn clone(&self, entity: Entity) -> Option<EntityBuilder> {
		let entity = self.data.entity(entity).ok()?;
		let mut builder = EntityBuilder::new();

		// haha no macro moment
		if let Some(comp) = entity.get::<PrototypeComp>() {
			builder.add((*comp).clone());
		}

		if let Some(comp) = entity.get::<PositionComp>() {
			builder.add((*comp).clone());
		}

		if let Some(comp) = entity.get::<HitboxComp>() {
			builder.add((*comp).clone());
		}

		if let Some(comp) = entity.get::<PhysicsComp>() {
			builder.add((*comp).clone());
		}

		if let Some(comp) = entity.get::<GravityComp>() {
			builder.add((*comp).clone());
		}

		if let Some(comp) = entity.get::<HumanoidComp>() {
			builder.add((*comp).clone());
		}

		Some(builder)
	}
}

pub struct EntitySystem {
	storage: EntityStorage,

	physics_system: PhysicsECSystem,
	gravity_system: GravityECSystem,
	movement_system: MovementECSystem,
	collision_system: CollisionECSystem,

	// Keep out of reach of children
	dead: HashSet<Entity>,
}

impl EntitySystem {
	pub fn new() -> EntitySystem {
		EntitySystem {
			storage: EntityStorage {
				data: Default::default(),
			},
			physics_system: Default::default(),
			gravity_system: GravityECSystem::new(),
			movement_system: Default::default(),
			collision_system: Default::default(),
			dead: Default::default(),
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

	pub fn tick(&mut self, chunks: &ChunkSystem, delta: f32) -> Result<()> {
		self.gravity_system.tick(&mut self.storage, delta);
		self.movement_system.tick(&mut self.storage, delta);
		self.collision_system.tick(&mut self.storage, chunks, delta);
		self.physics_system.tick(&mut self.storage, delta);
		Ok(())
	}
}

impl Deref for EntitySystem {
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
