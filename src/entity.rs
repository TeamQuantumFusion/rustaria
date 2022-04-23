pub mod component;
pub mod world;

use rustaria_util::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Entity {
	uuid: Uuid,
}
