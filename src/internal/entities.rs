use std::ops::{Deref, DerefMut};

use rustaria_api::ty::RawId;
use rustaria_api::{Carrier, Reloadable};
use rustaria_common::error::{ContextCompat, Result};
use rustaria_common::math::{Vector2D, WorldSpace};
use rustaria_common::Uuid;
use rustaria_network::Token;

use crate::api::prototype::entity::EntityPrototype;
use crate::entity::world::EntityWorld;
use crate::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::packet::ServerPacket;
use crate::{ChunkSystem, NetworkSystem, Server, SmartError};

pub(crate) struct EntitySystem {
	carrier: Option<Carrier>,
	world: EntityWorld,
	new_entities: Vec<(Uuid, RawId, Vector2D<f32, WorldSpace>)>,
}

impl EntitySystem {
	pub fn new() -> EntitySystem {
		EntitySystem {
			carrier: None,
			new_entities: vec![],
			world: EntityWorld::default(),
		}
	}

	pub fn spawn(&mut self, id: RawId, position: Vector2D<f32, WorldSpace>) -> Result<Uuid> {
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

	#[macro_module::module(server.entity)]
	pub fn tick(this: &mut EntitySystem, server: &mut Server) -> Result<()> {
		for id in &this.world.dead {
			server.network.send_all(ServerPacket::Entity(ServerEntityPacket::Kill(*id)))?;
		}

		this.world.tick(&server.chunk)?;
		for (uuid, id, pos) in this.new_entities.drain(..) {
			server.network.send_all(ServerPacket::Entity(ServerEntityPacket::New(uuid, id, pos)))?;
		}

		Ok(())
	}

	pub fn packet(&mut self, _: Token, packet: ClientEntityPacket) -> Result<()> {
		match packet {
			ClientEntityPacket::Spawn(id, pos) => {
				self.spawn(id, pos)?;
			}
		}

		Ok(())
	}
}

impl Reloadable for EntitySystem {
	fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
	}
}

impl Deref for EntitySystem {
	type Target = EntityWorld;

	fn deref(&self) -> &Self::Target {
		&self.world
	}
}

impl DerefMut for EntitySystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.world
	}
}
