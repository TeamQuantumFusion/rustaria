use eyre::Result;
use hecs::{Component, DynamicBundle, DynamicBundleClone, Entity, EntityBuilder, EntityBuilderClone, EntityRef, Query, QueryBorrow, QueryMut, Ref, RefMut, TakenEntity};

use crate::{
	api::Api,
	debug::DebugRendererImpl,
	iter_components,
	ty::id::Id,
	world::entity::{
		prototype::EntityDesc,
		system::{
			collision::CollisionSystem, humanoid::HumanoidSystem, GravitySystem, VelocitySystem,
		},
	},
	ChunkStorage,
};
use crate::world::entity::system::network::{EntityPacket, NetworkSystem};

pub mod component;
pub mod prototype;
pub mod system;

pub struct EntityStorage {
	world: hecs::World,
}

impl EntityStorage {
	pub fn new() -> EntityStorage {
		EntityStorage {
			world: Default::default(),
		}
	}

	pub fn push(&mut self, api: &Api, id: Id<EntityDesc>) -> Entity {
		self.world.spawn(&api.carrier.entity.get(id).template)
	}

	pub fn insert(&mut self, api: &Api, entity: Entity, id: Id<EntityDesc>) {
		self.world
			.spawn_at(entity, &api.carrier.entity.get(id).template)
	}

	pub fn put_comp(&mut self, entity: Entity, components: impl DynamicBundle) {
		self.world.spawn_at(entity, components)
	}

	pub fn remove(&mut self, entity: Entity) -> Option<TakenEntity<'_>> {
		self.world.take(entity).ok()
	}

	pub fn get(&self, entity: Entity) -> Option<EntityRef<'_>> { self.world.entity(entity).ok() }

	pub fn contains(&self, entity: Entity) -> bool { self.world.contains(entity) }

	pub fn get_comp<T: Component>(&self, entity: Entity) -> Option<Ref<'_, T>> {
		self.world.get(entity).ok()
	}

	pub fn get_mut_comp<T: Component>(&mut self, entity: Entity) -> Option<RefMut<'_, T>> {
		self.world.get_mut(entity).ok()
	}

	pub fn query<Q: Query>(&self) -> QueryBorrow<'_, Q> { self.world.query() }

	pub fn query_mut<Q: Query>(&mut self) -> QueryMut<'_, Q> { self.world.query_mut() }

	pub fn clone(&self, entity: Entity) -> Option<EntityBuilder> {
		let entity = self.world.entity(entity).ok()?;
		let mut builder = EntityBuilder::new();
		iter_components!({
			if let Some(component) = entity.get::<T>() {
				builder.add((*component).clone());
			}
		});

		Some(builder)
	}

	pub fn clone_to(&self, from: Entity, to: Entity, to_storage: &mut EntityStorage) -> Option<()> {
		let entity = self.world.entity(from).ok()?;
		iter_components!({
			if let Some(component) = entity.get::<T>() {
				to_storage.world.insert_one(to,  (*component).clone()).ok()?;
			}
		});

		Some(())
	}
}

pub struct EntityWorld {
	pub storage: EntityStorage,
	velocity: VelocitySystem,
	gravity: GravitySystem,
	collision: CollisionSystem,
	humanoid: HumanoidSystem,
	network: NetworkSystem,
}

impl EntityWorld {
	pub fn new(api: &Api) -> Result<EntityWorld> {
		Ok(EntityWorld {
			storage: EntityStorage::new(),
			velocity: VelocitySystem,
			gravity: GravitySystem,
			collision: CollisionSystem,
			humanoid: HumanoidSystem,
			network: NetworkSystem
		})
	}

	pub fn tick(&mut self, api: &Api, chunks: &ChunkStorage, debug: &mut impl DebugRendererImpl) {
		self.gravity.tick(&mut self.storage);
		self.humanoid.tick(&mut self.storage);
		self.collision.tick(api, &mut self.storage, chunks, debug);
		self.velocity.tick(&mut self.storage, debug);
	}

	pub fn packet(&mut self, packet: &EntityPacket) {
		self.network.apply(&mut self.storage, packet);
	}
}
