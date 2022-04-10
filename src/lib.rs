pub mod api;
pub mod network;
pub mod world;

pub const UPS: usize = 20;

use crate::api::Api;
use crate::network::packet::{ClientPacket, ServerPacket};
use crate::network::Networking;
use crate::world::World;

pub struct Server {
    pub api: Api,
    pub network: Networking,
    pub world: World,
}

impl Server {
    pub fn tick(&mut self) {
        self.world.tick(&mut self.network);
    }
}
