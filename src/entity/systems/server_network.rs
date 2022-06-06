use hecs::Entity;
use std::collections::HashMap;

use rsa_core::error::Result;
use rsa_core::logging::debug;

use rsa_network::Token;

use crate::entity::component::humanoid::HumanoidComp;

use crate::entity::packet::ClientEntityPacket;
use crate::entity::EntityStorage;

#[derive(Default)]
pub(crate) struct ServerNetworkECSystem {
	position_queue: Vec<(u32, Entity)>,
	player_entities: HashMap<Token, Entity>,
}

impl ServerNetworkECSystem {
	//pub(crate) fn tick(
	//	&mut self,
	//	world: &mut World,
	//	network: &mut ServerTunnel<ServerEntityPacket>,
	//) -> Result<()> {
	//	for (tick, entity) in self.position_queue.drain(..) {
	//		if let Ok(component) = world.get_mut::<&mut PositionComp>(entity) {
	//			network.send(ServerEntityPacket::Pos(tick, entity, component.position))?;
	//		}
	//	}
	//	Ok(())
	//}

	pub(crate) fn packet(
		&mut self,
		storage: &mut EntityStorage,
		token: &Token,
		packet: ClientEntityPacket,
	) -> Result<()> {
		todo!();
		match packet {
			ClientEntityPacket::RequestPos(tick, entity) => {
				self.position_queue.push((tick, entity));
			}
			ClientEntityPacket::PlayerDirection(tick, dir) => {
				// Check if its actually the correct player entity.
				if let Some(entity) = self.player_entities.get(token) {
					if let Ok(comp) = storage.get_mut::<&mut HumanoidComp>(*entity) {
						if dir.x > 1.0 || dir.x < 0.0 || dir.y > 1.0 || dir.y < 0.0 {
							// Out of bounds
							//return Err(Report::new(NetworkError::Hacking(HackingInfraction::Imposter)));
						}
						comp.direction = dir;
						// Wants response back on move
						self.position_queue.push((tick, *entity));
						return Ok(());
					} else {
						//debug!("Player entity {entity} for player {token} does not have a component.");
					}
				} else {
					debug!("Player on {token} does not have a player entity.");
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
