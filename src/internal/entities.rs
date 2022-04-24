use eyre::ContextCompat;
use std::ops::{Deref, DerefMut};

use rustaria_api::ty::RawId;
use rustaria_api::{Carrier, Reloadable};
use rustaria_network::Token;
use rustaria_util::math::{Vector2D, WorldSpace};
use rustaria_util::Uuid;

use crate::api::prototype::entity::EntityPrototype;
use crate::entity::world::EntityWorld;
use crate::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::packet::ServerPacket;
use crate::{ChunkManager, NetworkManager, SmartError};

pub(crate) struct EntityManager {
	carrier: Option<Carrier>,
	world: EntityWorld,
	new_entities: Vec<(Uuid, RawId, Vector2D<f32, WorldSpace>)>,
}

impl EntityManager {
	pub fn new() -> EntityManager {
		EntityManager {
			carrier: None,
			new_entities: vec![],
			world: EntityWorld::default(),
		}
	}

	pub fn spawn(&mut self, id: RawId, position: Vector2D<f32, WorldSpace>) -> eyre::Result<Uuid> {
		let carrier = self
			.carrier
			.as_ref()
			.wrap_err(SmartError::CarrierUnavailable)?
			.lock();

		// Get uuid and handle conflicts by re-rolling until you find a spot.
		let mut uuid = Uuid::new_v4();
		while self.world.entities.contains_key(&uuid) {
			uuid = Uuid::new_v4();
		}

		// Get prototype
		let prototype = carrier
			.get_registry::<EntityPrototype>()
			.prototype_from_id(id)
			.wrap_err("Could not find entity_manager")?;

		self.world.insert(uuid, id, position, prototype);
		self.new_entities.push((uuid, id, position));
		Ok(uuid)
	}

	pub fn tick(
		&mut self,
		chunks: &ChunkManager,
		network: &mut NetworkManager,
	) -> eyre::Result<()> {
		for id in &self.world.dead {
			network.send_all(ServerPacket::Entity(ServerEntityPacket::Kill(*id)))?;
		}

		self.world.tick(chunks)?;
		for (uuid, id, pos) in self.new_entities.drain(..) {
			network.send_all(ServerPacket::Entity(ServerEntityPacket::New(uuid, id, pos)))?;
		}

		Ok(())
	}

	pub fn packet(&mut self, _: Token, packet: ClientEntityPacket) -> eyre::Result<()> {
		match packet {
			ClientEntityPacket::Spawn(id, pos) => {
				self.spawn(id, pos)?;
			}
		}

		Ok(())
	}
}

impl Reloadable for EntityManager {
	fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
	}
}

impl Deref for EntityManager {
	type Target = EntityWorld;

	fn deref(&self) -> &Self::Target {
		&self.world
	}
}

impl DerefMut for EntityManager {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.world
	}
}
