use std::sync::Arc;

use legion::Entity;

use rustaria_api::{Carrier, Reloadable};
use rustaria_api::ty::RawId;
use rustaria_network::Token;
use rustaria_util::ty::pos::Pos;

use crate::{ServerPacket, ThreadPool};
use crate::entity::EntityContainer;
use crate::manager::network::NetworkManager;
use crate::network::packet::entity::{ClientEntityPacket, ServerEntityPacket};

pub(crate) struct EntityManager {
    container: EntityContainer,
    new_entities: Vec<(Token, RawId, Pos)>,
}

impl EntityManager {
    pub fn new(thread_pool: Arc<ThreadPool>) -> EntityManager {
        EntityManager {
            container: EntityContainer::new(thread_pool),
            new_entities: vec![],
        }
    }

    pub fn spawn(&mut self, from: Token, id: RawId, position: Pos) -> eyre::Result<Entity> {
        let entity = self.container.spawn(id, position)?;
        self.new_entities.push((from, id, position));
        Ok(entity)
    }

    pub fn tick(&mut self, network: &mut NetworkManager) -> eyre::Result<()> {
        self.container.tick();
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
        self.container.reload(api, carrier);
    }
}
