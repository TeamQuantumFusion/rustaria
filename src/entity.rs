use serde::{Deserialize, Serialize};

use rustaria_common::Uuid;

pub mod component;
pub mod world;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Entity {
	uuid: Uuid,
}
