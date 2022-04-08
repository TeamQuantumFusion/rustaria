pub mod network;
pub mod api;
pub mod world;

use rustaria_world::World;
use crate::api::Api;
use crate::network::Networking;

pub struct Server {
	pub api: Api,
	pub network: Networking,
	pub world: World
}