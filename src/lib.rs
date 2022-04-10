use rustaria_util::Result;

use crate::api::Api;
use crate::network::Networking;
use crate::network::packet::{ClientPacket, ServerPacket};
use crate::world::World;

pub mod api;
pub mod network;
pub mod world;

pub const UPS: usize = 20;

pub struct Server {
    pub api: Api,
    pub network: Networking,
    pub world: World,
}

impl Server {
    pub fn tick(&mut self) -> Result<()> {
        self.world.tick(&mut self.network)
    }
}
