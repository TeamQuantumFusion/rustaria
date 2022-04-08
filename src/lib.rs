pub mod network;
pub mod api;
pub mod world;

use crate::network::Networking;
use crate::network::packet::{ClientPacket, ServerPacket};
use crate::world::World;

pub struct Server {
	pub network: Networking,
	pub world: World
}

impl Server  {

	pub fn tick(&mut self) {
		self.world.tick(&mut self.network);
	}
}
