use std::collections::hash_map::Entry;
use std::collections::HashMap;
use rsa_core::ty::{Id, Identifier};
use rsa_core::err::Result;
use rsa_core::log::{debug, info, trace, warn};
use rsa_core::math::vec2;
use rsa_network::server::ServerSender;
use rsa_network::Token;
use rsa_player::packet::{ClientBoundPlayerPacket, ServerBoundPlayerPacket};
use rsa_world::entity::{Entity, EntityRef, EntityWorld};
use rsa_world::entity::component::{HumanoidComponent, PositionComponent};
use rsa_world::entity::prototype::EntityDesc;
use rsa_world::rpc::WorldRPC;
use rsa_world::World;

pub struct PlayerSystem {
	players: HashMap<Token, Option<Entity>>,
	response_requests: Vec<(u32, Token)>,
	joined: Vec<(Token, Entity)>,
	player_entity: Id<EntityDesc>,
}

impl PlayerSystem {
	pub fn new(rpc: &WorldRPC) -> Result<PlayerSystem> {
		info!("Initializing");
		Ok(PlayerSystem {
			players: Default::default(),
			response_requests: vec![],
			joined: Default::default(),
			player_entity: rpc
				.entity
				.get_id_from_identifier(&Identifier::new("player"))
				.ok_or("Could not find Player entity")?,
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

	pub fn tick(
		&mut self,
		sender: &mut ServerSender<ClientBoundPlayerPacket>,
		world: &World,
	) -> Result<()> {
		for (token, entity) in self.joined.drain(..) {
			debug!("Sent joined packet");
			sender.send(token, ClientBoundPlayerPacket::Joined(entity))?;
		}

		let responses: Vec<_> = self.response_requests.drain(..).collect();
		for (tick, token) in responses {
			sender.send(
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
		rpc: &WorldRPC,
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
				let entity = world.entities.storage.push(rpc, self.player_entity);
				self.players.insert(token, Some(entity));
				self.joined.push((token, entity));
			}
			ServerBoundPlayerPacket::PlaceBlock(pos, layer_id, block_id) => {
				world.place_block(rpc, pos, layer_id, block_id);
			}
		}
	}
}
