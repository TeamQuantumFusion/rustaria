use std::sync::Arc;

use eyre::{Result, ContextCompat};
use legion::{Entity, Resources, Schedule};
use rayon::ThreadPool;
use rustaria_api::{Carrier, Reloadable};
use rustaria_api::ty::{RawId, Prototype};
use serde::Deserialize;

use rustaria_network::Token;
use rustaria_util::ty::pos::Pos;

pub use legion::*;

use crate::err::SmartError;
use crate::{Networking, ServerPacket};
use crate::api::prototype::entity::EntityPrototype;

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
            velocity: Pos { x: 0.0, y: 0.0 }
        }
    }
}

#[legion::system(for_each)]
pub fn update_positions(pos: &mut PositionComp, vel: &VelocityComp) {
    pos.position += vel.velocity;
}

pub struct EntityHandler {
    carrier: Option<Carrier>,
    pub universe: Universe,
    schedule: Schedule,
    resources: Resources,
    thread_pool: Arc<ThreadPool>,

    new_entities: Vec<(RawId, Pos)>
}

impl EntityHandler {
    pub fn new(thread_pool: Arc<ThreadPool>) -> EntityHandler {
        EntityHandler {
            carrier: None,
            universe: Universe::default(),
            resources: Resources::default(),
            schedule: Schedule::builder()
                .add_system(update_positions_system())
                .build(),
            thread_pool,
            new_entities: vec![]
        }
    }

    pub fn spawn(&mut self, id: RawId, position: Pos) -> Result<Entity> {
        let carrier = self.carrier.as_ref().wrap_err(SmartError::CarrierUnavailable)?;
        // Create entity and get its entry to add dynamic components.
        let entity = self.universe.push((IdComp(id), PositionComp { position }));
        let mut entry = self.universe.entry(entity).unwrap();

        // Get instance, get prototype and add all of the needed components.
        let instance = carrier.lock();
        let prototype = instance.get_registry::<EntityPrototype>().get_prototype(id).wrap_err("Could not find entity")?;
        if let Some(velocity) = &prototype.velocity {
            entry.add_component(velocity.create(id));
        }

        self.new_entities.push((id, position));

        Ok(entity)
    }

    pub fn tick(&mut self, network: &mut Networking) {
        self.schedule.execute_in_thread_pool(
            &mut self.universe,
            &mut self.resources,
            &self.thread_pool,
        );

        for (id, pos) in self.new_entities.drain(..) {
            network.internal.distribute(Token::nil(), ServerPacket::NewEntity(id, pos)).unwrap();
        }
    }
}

impl Reloadable for EntityHandler {
    fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
        self.carrier = Some(carrier.clone());
    }
}