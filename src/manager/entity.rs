use std::sync::Arc;

use legion::Entity;

use rustaria_api::ty::RawId;
use rustaria_api::{Carrier, Reloadable};
use rustaria_network::Token;
use rustaria_util::ty::pos::Pos;

use crate::entity::EntityContainer;
use crate::manager::network::NetworkManager;
use crate::network::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use crate::{ServerPacket, ThreadPool};

pub(crate) struct EntityManager {
	container: EntityContainer,
	new_entities: Vec<(RawId, Pos)>,
}

impl EntityManager {
	pub fn new(thread_pool: Arc<ThreadPool>) -> EntityManager {
		EntityManager {
			container: EntityContainer::new(thread_pool),
			new_entities: vec![],
		}
	}

	#[allow(unused)]
	pub fn spawn(&mut self, id: RawId, position: Pos) -> eyre::Result<Entity> {
		let entity = self.container.spawn(id, position)?;
		self.new_entities.push((id, position));
		Ok(entity)
	}

	pub fn tick(&mut self, network: &mut NetworkManager) -> eyre::Result<()> {
		self.container.tick();
		for (id, pos) in self.new_entities.drain(..) {
			network.internal.distribute(
				Token::nil(),
				ServerPacket::Entity(ServerEntityPacket::Spawn(id, pos)),
			)?;
		}

		Ok(())
	}

	pub fn packet(&mut self, _: Token, packet: ClientEntityPacket) {
		match packet {}
	}
}

impl Reloadable for EntityManager {
	fn reload(&mut self, api: &rustaria_api::Api, carrier: &Carrier) {
		self.container.reload(api, carrier);
	}
}
