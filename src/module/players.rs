use std::collections::HashMap;

use rsa_core::ty::{RawId, Tag};
use rsa_core::api::{Api, Reloadable};
use rsa_core::error::{ContextCompat, Result};
use rsa_core::logging::{info, warn};
use rsa_core::math::vec2;
use rsa_network::Token;

use crate::api::prototype::entity::EntityPrototype;
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use crate::player::Player;
use crate::{EntitySystem, NetworkSystem, ServerPacket, SmartError};

pub struct PlayerSystem {
	player_entity: Option<RawId>,
	players: HashMap<Token, Player>,
}

impl PlayerSystem {
	pub fn new() -> PlayerSystem {
		PlayerSystem {
			player_entity: None,
			players: Default::default(),
		}
	}

	pub fn join(&mut self, token: Token) {
		info!("Player joined {}", token);
		self.players.insert(token, Player::new(token.to_string()));
	}

	pub fn packet(
		&mut self,
		from: Token,
		packet: ClientPlayerPacket,
		entities: &mut EntitySystem,
		network: &NetworkSystem,
	) -> Result<()> {
		if let Some(player) = self.players.get_mut(&from) {
			match packet {
				// this should prob be world spawn later on
				ClientPlayerPacket::Join { .. } => {
					let player_entity = self
						.player_entity
						.wrap_err(SmartError::CarrierUnavailable)?;
					let pos = vec2(3.0, 20.0);
					let entity = entities.spawn(player_entity, pos)?;
					network.send(
						from,
						ServerPacket::Player(ServerPlayerPacket::Attach { entity, pos }),
					)?;

					player.entity = Some(entity);
				}
				ClientPlayerPacket::SetPos(pos) => {
					if let Some(entity) = player.entity {
						if let Some(position) = entities.position.get_mut(&entity) {
							position.position = pos;
						}
					}
				}
			};
		} else {
			warn!("Player {from} does not exist.");
		}

		Ok(())
	}
}

impl Reloadable for PlayerSystem {
	fn reload(&mut self, api: &Api) {
		self.player_entity = api.get_carrier()
			.get::<EntityPrototype>()
			.id_from_tag(&Tag::rsa("player"))
	}
}
