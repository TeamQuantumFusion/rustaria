use std::collections::HashMap;

use rsa_core::api::Api;
use rsa_core::error::Result;
use rsa_core::logging::info;
use rsa_core::ty::{RawId, Tag};
use rsa_network::Token;

use crate::{EntityModule, NetworkModule};
use crate::entity::prototype::EntityPrototype;
use crate::packet::player::ClientPlayerPacket;
use crate::player::Player;

pub struct PlayerModule {
	player_entity: Option<RawId>,
	players: HashMap<Token, Player>,
}

impl PlayerModule {
	pub fn new() -> PlayerModule {
		PlayerModule {
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
		entities: &mut EntityModule,
		network: &NetworkModule,
	) -> Result<()> {
		todo!();
		//if let Some(player) = self.players.get_mut(&from) {
		//	match packet {
		//		// this should prob be world spawn later on
		//		ClientPlayerPacket::Join { .. } => {
		//			let player_entity = self
		//				.player_entity
		//				.wrap_err(SmartError::CarrierUnavailable)?;
		//			let pos = vec2(3.0, 20.0);
		//			let entity = entities.spawn(player_entity, pos)?;
		//			network.send(
		//				from,
		//				ServerPacket::Player(ServerPlayerPacket::Attach { entity, pos }),
		//			)?;

		//			player.entity = Some(entity);
		//		}
		//		ClientPlayerPacket::SetPos(pos) => {
		//			if let Some(entity) = player.entity {
		//				if let Some(position) = entities.position.get_mut(&entity) {
		//					position.position = pos;
		//				}
		//			}
		//		}
		//	};
		//} else {
		//	warn!("Player {from} does not exist.");
		//}

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
