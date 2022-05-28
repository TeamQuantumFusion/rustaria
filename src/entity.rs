use serde::{Deserialize, Serialize};
use rsa_core::ty::Uuid;


pub mod component;
pub mod world;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Entity {
	uuid: Uuid,
}
