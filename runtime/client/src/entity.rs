use eyre::Result;
use rustaria::network::packet::entity::ServerEntityPacket;
use rustaria_api::{Api, Carrier, Reloadable};

pub(crate) struct EntityHandler {}

impl EntityHandler {
    pub fn packet(&mut self, _: ServerEntityPacket) -> Result<()> {
        //match packet {
        //     ServerEntityPacket::Spawn(_, _) => todo!(),
        // }

        Ok(())
    }
}

impl Reloadable for EntityHandler {
    fn reload(&mut self, _: &Api, _: &Carrier) {}
}
