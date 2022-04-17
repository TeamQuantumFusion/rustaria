use std::sync::Arc;

use rustaria_api::ty::RawId;
use rustaria_api::{Carrier, Reloadable};
use rustaria_network::Token;
use rustaria_util::ty::pos::Pos;
use rustaria_util::Uuid;

use crate::entity::EntityWorld;
use crate::manager::network::NetworkManager;
use crate::network::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::{ServerPacket, ThreadPool};

pub(crate) struct EntityManager {
	world: EntityWorld,
	new_entities: Vec<(Token, RawId, Pos)>,
}

impl EntityManager {
	pub fn new(thread_pool: Arc<ThreadPool>) -> EntityManager {
		EntityManager {
			world: EntityWorld::default(),
			new_entities: vec![],
		}
	}

	pub fn spawn(&mut self, from: Token, id: RawId, position: Pos) -> eyre::Result<Uuid> {
		let entity = self.world.spawn(id, position)?;
		self.new_entities.push((from, id, position));
		Ok(entity)
	}

	pub fn tick(&mut self, network: &mut NetworkManager) -> eyre::Result<()> {
		self.world.tick();
		for (from, id, pos) in self.new_entities.drain(..) {
			network
				.internal
				.distribute(from, ServerPacket::Entity(ServerEntityPacket::New(id, pos)))?;
		}

		Ok(())
	}

	pub fn packet(&mut self, from: Token, packet: ClientEntityPacket) -> eyre::Result<()> {
		match packet {
			ClientEntityPacket::Spawn(id, pos) => {
				self.spawn(from, id, pos)?;
			}
		}

		Ok(())
	}
}

impl Reloadable for EntityManager {
	fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
		self.world.reload(api, carrier);
	}
}
