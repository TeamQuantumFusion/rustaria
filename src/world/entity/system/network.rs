use euclid::Vector2D;
use hecs::Entity;
use log::warn;

use crate::{
	ty::WS,
	world::entity::{
		component::{HumanoidComponent, PhysicsComponent, PositionComponent},
		EntityStorage,
	},
};

pub struct NetworkSystem;

impl NetworkSystem {
	pub fn apply(&mut self, storage: &mut EntityStorage, packet: &EntityPacket) {
		if let Some(entity) = storage.get(packet.entity) {
			match packet.component {
				EntityComponentPacket::Physics {
					add_velocity,
					add_accel,
				} => {
					if let Some(mut physics) = entity.get_mut::<PhysicsComponent>() {
						physics.vel += add_velocity;
						physics.accel += add_accel;
					}
				}
				EntityComponentPacket::Humanoid { dir, jumping } => {
					if let Some(mut comp) = entity.get_mut::<HumanoidComponent>() {
						comp.jumping = jumping;
						comp.dir = dir;
					}
				}
				EntityComponentPacket::Pos { set_pos } => {
					if let Some(mut comp) = entity.get_mut::<PositionComponent>() {
						comp.pos = set_pos;
					}
				}
			}
		} else {
			warn!("Entity {:?} does not exist", packet.entity);
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EntityPacket {
	pub entity: Entity,
	pub component: EntityComponentPacket,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum EntityComponentPacket {
	Pos {
		set_pos: Vector2D<f32, WS>,
	},
	Physics {
		add_velocity: Vector2D<f32, WS>,
		add_accel: Vector2D<f32, WS>,
	},
	Humanoid {
		dir: Vector2D<f32, WS>,
		jumping: bool,
	},
}
