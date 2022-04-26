use std::collections::HashMap;

use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_api::ty::{RawId, Tag};
use rustaria_network::Token;
use rustaria_util::error::{ContextCompat, Result};
use rustaria_util::logging::{info, warn};
use rustaria_util::math::vec2;

use crate::{EntityManager, NetworkManager, PlayerJoinData, ServerPacket, SmartError};
use crate::api::prototype::entity::EntityPrototype;
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use crate::player::Player;

pub(crate) struct PlayerManager {
	player_entity: Option<RawId>,
	players: HashMap<Token, Player>,
}

impl PlayerManager {
	pub fn new() -> PlayerManager {
		PlayerManager {
			player_entity: None,
			players: Default::default(),
		}
	}

	pub fn join(&mut self, player: Token, data: PlayerJoinData) {
		info!("Player joined {}", player);
		self.players.insert(
			player,
			Player {
				name: "Youyr nmom".to_string(),
				entity: None,
			},
		);
	}

	pub fn packet(
		&mut self,
		from: Token,
		packet: ClientPlayerPacket,
		entities: &mut EntityManager,
		network: &NetworkManager,
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

impl Reloadable for PlayerManager {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		let access = carrier.lock();
		self.player_entity = access
			.get_registry::<EntityPrototype>()
			.id_from_tag(&Tag::new("rustaria:player").unwrap())
	}
}
