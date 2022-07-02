use std::collections::{hash_map::Entry, HashMap};

use euclid::{vec2, Vector2D};
use anyways::Result;
use hecs::{Entity, EntityRef};
use tracing::{debug, info, trace, warn};

use crate::{
	api::Api,
	network::Token,
	packet,
	ty::{id::Id, identifier::Identifier, WS},
	world::entity::{
		component::{HumanoidComponent, PositionComponent},
		prototype::EntityDesc,
	},
	EntityWorld, ServerNetwork, World,
};

packet!(Player(ServerBoundPlayerPacket, ClientBoundPlayerPacket));

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerBoundPlayerPacket {
	SetMove(u32, PlayerCommand),
	Join(),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundPlayerPacket {
	RespondPos(u32, Option<Vector2D<f32, WS>>),
	Joined(Entity),
}

#[derive(Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerCommand {
	pub dir:     Vector2D<f32, WS>,
	pub jumping: bool,
}

pub(crate) struct PlayerSystem {
	players: HashMap<Token, Option<Entity>>,
	response_requests: Vec<(u32, Token)>,
	joined: Vec<(Token, Entity)>,
	player_entity: Id<EntityDesc>,
}

impl PlayerSystem {
	pub fn new(api: &Api) -> Result<PlayerSystem> {
		info!("Initializing");
		Ok(PlayerSystem {
			players: Default::default(),
			response_requests: vec![],
			joined: Default::default(),
			player_entity: api
				.carrier
				.entity
				.get_id(&Identifier::new("player")).ok_or("Could not find Player entity")?
		})
	}

	fn get_player_entity<'a, 'e>(
		&'a mut self,
		token: Token,
		entity_world: &'e EntityWorld,
	) -> Option<EntityRef<'e>> {
		match self.players.entry(token) {
			Entry::Occupied(mut occupied) => {
				if let Some(entity) = *occupied.get() {
					if let Some(entity) = entity_world.storage.get(entity) {
						return Some(entity);
					} else {
						warn!("Player entity got yeeted");
						(*occupied.get_mut()) = None;
					}
				}
			}
			Entry::Vacant(_) => {}
		}
		None
	}

	pub fn tick(&mut self, networking: &mut ServerNetwork, world: &World) -> Result<()> {
		for (token, entity) in self.joined.drain(..) {
			debug!("Sent joined packet");
			networking.send(token, ClientBoundPlayerPacket::Joined(entity))?;
		}

		let responses: Vec<_> = self.response_requests.drain(..).collect();
		for (tick, token) in responses {
			networking.send(
				token,
				ClientBoundPlayerPacket::RespondPos(
					tick,
					self.get_player_entity(token, &world.entities)
						.map(|entity| entity.get::<PositionComponent>().expect("where pos").pos),
				),
			)?;
		}

		Ok(())
	}

	pub fn packet(
		&mut self,
		api: &Api,
		token: Token,
		packet: ServerBoundPlayerPacket,
		world: &mut World,
	) {
		match packet {
			ServerBoundPlayerPacket::SetMove(tick, speed) => {
				if let Some(player) = self.get_player_entity(token, &world.entities) {
					let mut component = player
						.get_mut::<HumanoidComponent>()
						.expect("Player does not have velocity");
					component.dir = speed.dir.clamp(vec2(-1.0, -1.0), vec2(1.0, 1.0));
					component.jumping = speed.jumping;
				} else {
					trace!("player entity not here");
				}
				self.response_requests.push((tick, token));
			}
			ServerBoundPlayerPacket::Join() => {
				info!("Player {:?} joined", token);
				let entity = world.entities.storage.push(api, self.player_entity);
				self.players.insert(token, Some(entity));
				self.joined.push((token, entity));
			}
		}
	}
}

#[derive(Clone)]
pub struct Player {
	pub pos:      Vector2D<f32, WS>,
	pub velocity: Vector2D<f32, WS>,
}

impl Player {
	pub fn tick(&mut self, delta: f32) { self.pos += self.velocity * delta; }
}
