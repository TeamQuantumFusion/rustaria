use std::ops::{Deref, DerefMut};
use rsa_core::api::{Api, Reloadable};

use rsa_core::ty::{RawId, Uuid};
use rsa_core::api::carrier::Carrier;
use rsa_core::error::{ContextCompat, Result};
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_network::Token;

use crate::api::prototype::entity::EntityPrototype;
use crate::entity::world::EntityWorld;
use crate::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::packet::ServerPacket;
use crate::{Server, SmartError};

pub struct EntitySystem {
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
			.wrap_err(SmartError::CarrierUnavailable)?;

		// Get uuid and handle conflicts by re-rolling until you find a spot.
		let mut uuid = Uuid::new_v4();
		while self.world.entities.contains_key(&uuid) {
			uuid = Uuid::new_v4();
		}

		// Get prototype
		let prototype = carrier
			.get::<EntityPrototype>()
			.prototype_from_id(id)
			;

		self.world.insert(uuid, id, position, prototype);
		self.new_entities.push((uuid, id, position));
		Ok(uuid)
	}

	#[macro_module::module(server.entity)]
	pub fn tick(this: &mut EntitySystem, server: &mut Server) -> Result<()> {
		for id in &this.world.dead {
			server
				.network
				.send_all(ServerPacket::Entity(ServerEntityPacket::Kill(*id)))?;
		}

		this.world.tick(&server.chunk)?;
		for (uuid, id, pos) in this.new_entities.drain(..) {
			server
				.network
				.send_all(ServerPacket::Entity(ServerEntityPacket::New(uuid, id, pos)))?;
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
	fn reload(&mut self, api: &Api) {
		self.carrier = Some(api.get_carrier());
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
