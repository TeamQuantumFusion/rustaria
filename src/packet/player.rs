use serde::{Deserialize, Serialize};
use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::ty::Uuid;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerPlayerPacket {
	Attach {
		entity: Uuid,
		pos: Vector2D<f32, WorldSpace>,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientPlayerPacket {
	/// Creates a Player Entity
	Join(),
	SetPos(Vector2D<f32, WorldSpace>),
}
