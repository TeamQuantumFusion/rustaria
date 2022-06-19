use std::collections::HashMap;

use rsa_core::api::Api;
use rsa_core::error::{ContextCompat, Result};
use rsa_core::logging::{info, warn};
use rsa_core::math::vec2;
use rsa_core::ty::{RawId, Tag};
use rsa_network::Token;

use crate::entity::prototype::EntityPrototype;
use crate::packet::player::{ClientPlayerPacket, ServerPlayerPacket};
use crate::player::Player;
use crate::{CarrierUnavailable, NetworkModule, ServerPacket, World};
use crate::entity::packet::ServerEntityPacket;

pub struct PlayerModule {
	api: Api,
	player_entity: Option<RawId>,
	players: HashMap<Token, Player>,
}

impl PlayerModule {
	pub fn new(api: &Api) -> PlayerModule {
		PlayerModule {
			api: api.clone(),
			player_entity: None,
			players: Default::default(),
		}
	}

	pub fn get_player(&self, token: &Token) -> Option<&Player> {
		self.players.get(token)
	}

	pub fn join(&mut self, token: Token) {
		info!("Player joined {}", token);
		self.players.insert(token, Player::new(token.to_string()));
	}

	pub fn packet(
		&mut self,
		from: Token,
		packet: ClientPlayerPacket,
		world: &mut World,
		network: &NetworkModule,
	) -> Result<()> {
		if let Some(player) = self.players.get_mut(&from) {
			match packet {
				// this should prob be world spawn later on
				ClientPlayerPacket::Join { .. } => {
					let id = self.player_entity.wrap_err(CarrierUnavailable)?;
					let pos = vec2(3.0, 50.0);
					let entity = world.entities.spawn(
						pos,
						id,
						self.api
							.get_carrier()
							.get::<EntityPrototype>()
							.prototype_from_id(id),
					);

					network.send(from, ServerPacket::Entity(ServerEntityPacket::Spawn(0, entity, id)))?;
					network.send(from, ServerPacket::Entity(ServerEntityPacket::Pos(0, entity, pos)))?;
					network.send(from, ServerPacket::Player(ServerPlayerPacket::Attach { entity }), )?;

					player.entity = Some(entity);
				}
			};
		} else {
			warn!("Player {from} does not exist.");
		}

		Ok(())
	}

	pub fn reload(&mut self, api: &Api) {
		self.player_entity = Some(
			api.get_carrier()
				.get::<EntityPrototype>()
				.id_from_tag(&Tag::rsa("player"))
				.unwrap(),
		);
	}
}
