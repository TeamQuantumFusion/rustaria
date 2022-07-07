use std::collections::{hash_map::Entry, HashMap};

use rsa_core::{
	err::Result,
	log::{debug, info, trace, warn},
	math::{vec2, Vector2D},
	ty::{Id, Identifier, WS},
};
use rsa_network::{
	server::{ServerNetwork, ServerSender},
	Token,
};
use rsa_world::{
	ClientBoundWorldPacket,
	entity::{
		component::{HumanoidComponent, PositionComponent},
		Entity,
		EntityRef, EntityWorld, prototype::EntityDesc,
	}, World,
};
use rsa_world::rpc::WorldRPC;

use crate::packet::{ClientBoundPlayerPacket, ServerBoundPlayerPacket};

pub mod packet;

#[derive(Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerCommand {
	pub dir: Vector2D<f32, WS>,
	pub jumping: bool,
}

#[derive(Clone)]
pub struct Player {
	pub pos: Vector2D<f32, WS>,
	pub velocity: Vector2D<f32, WS>,
}

impl Player {
	pub fn tick(&mut self, delta: f32) { self.pos += self.velocity * delta; }
}
