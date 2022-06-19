use hecs::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPlayerPacket {
	/// Responds to the player what entity to attach to and create.
	Attach {
		entity: Entity,
	},

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientPlayerPacket {
	/// Creates a Player Entity
	Join(),
}