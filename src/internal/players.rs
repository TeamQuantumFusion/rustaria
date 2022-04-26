use std::collections::HashMap;

use rustaria_api::ty::{RawId, Tag};
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_common::error::{ContextCompat, Result};
use rustaria_common::logging::{info, warn};
use rustaria_common::math::vec2;
use rustaria_network::Token;

use crate::api::prototype::entity::EntityPrototype;
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use crate::player::Player;
use crate::{EntitySystem, NetworkSystem, PlayerJoinData, ServerPacket, SmartError};

pub(crate) struct PlayerSystem {
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

	pub fn join(&mut self, player: Token, data: PlayerJoinData) {
		info!("Player joined {}", player);
		self.players.insert(player, data.player);
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
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		let access = carrier.lock();
		self.player_entity = access
			.get_registry::<EntityPrototype>()
			.id_from_tag(&Tag::new("rustaria:player").unwrap())
	}
}
