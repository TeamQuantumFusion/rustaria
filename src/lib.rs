use eyre::Result;
use rustaria_api::{Reloadable, Api, Carrier};
use rustaria_util::debug;

use crate::network::Networking;
use crate::network::packet::{ClientPacket, ServerPacket};
use crate::world::World;

pub mod api;
pub mod network;
pub mod err;
pub mod world;

pub const UPS: usize = 20;

pub struct Server {
    pub network: Networking,
    pub world: World,
}

impl Server {
    pub fn tick(&mut self) -> Result<()> {
        self.world.tick(&mut self.network)
    }
}

impl Reloadable for Server {
    fn reload(&mut self, api: &Api, carrier: &Carrier) {
        debug!("Reloading Server");
        self.world.reload(api, carrier);
    }
}