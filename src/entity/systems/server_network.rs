use hecs::Entity;
use std::collections::HashMap;

use rsa_core::error::Result;
use rsa_core::logging::{debug, error};

use rsa_network::Token;
use rsa_network::tunnel::Tunnel;

use crate::entity::component::humanoid::HumanoidComp;

use crate::entity::packet::{ClientEntityPacket, ServerEntityPacket};
use crate::entity::EntityStorage;
use crate::{PlayerModule, ServerTunnel};
use crate::entity::component::pos::PositionComp;

#[derive(Default)]
pub(crate) struct ServerNetworkECSystem {
	position_queue: Vec<(u32, Entity)>,
}

impl ServerNetworkECSystem {
	pub(crate) fn tick(
		&mut self,
		world: &mut EntityStorage,
		network: &mut ServerTunnel<ServerEntityPacket>,
	) -> Result<()> {
		for (tick, entity) in self.position_queue.drain(..) {
			if let Ok(component) = world.get::<PositionComp>(entity) {
				network.send(ServerEntityPacket::Pos(tick, entity, component.position))?;
			}
		}
		Ok(())
	}

	pub(crate) fn packet(
		&mut self,
		players: &PlayerModule,
		storage: &mut EntityStorage,
		token: &Token,
		packet: ClientEntityPacket,
	) -> Result<()> {
		match packet {
			ClientEntityPacket::RequestPos(tick, entity) => {
				self.position_queue.push((tick, entity));
			}
			ClientEntityPacket::PlayerDirection(tick, dir) => {
				// Check if its actually the correct player entity.
				if let Some(player) = players.get_player(token) {
					if let Some(entity) = &player.entity {
						if let Ok(mut comp) = storage.get_mut::<HumanoidComp>(*entity) {
							if dir.x > 1.0 || dir.x < -1.0 || dir.y > 1.0 || dir.y < -1.0 {
								// Out of bounds
								//return Err(Report::new(NetworkError::Hacking(HackingInfraction::Imposter)));
								error!("haccks {:?}", dir);
							}
							comp.dir = dir;
							// Wants response back on move
							self.position_queue.push((tick, *entity));
							return Ok(());
						} else {
							debug!("Player entity {entity:?} for player {token} does not have a HumanoidComp.");
						}
					} else {
						debug!("Player on {token} does not have a player entity.");
					}
				}  else {
					debug!("Player does not exist.");

				}
			}
		}

		Ok(())
	}
}

#[derive(Debug)]
pub enum HackingInfraction {
	// For minor errors that may just be caused by a de-sync
	Sus,
	// For bigger sus marks that they are definitely hacking but does not pose a risk for the rustaria kernel.
	Imposter,
	// Should instantly be kicked. DDOS level shit.
	Emergency,
}

#[derive(Debug)]
pub enum NetworkError {
	Warn,
	Hacking(HackingInfraction),
}
