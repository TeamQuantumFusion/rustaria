use serde::{Deserialize, Serialize};

use packet::{ClientPacket, ServerPacket};
use rustaria_util::Result;
use rustaria_network::local::LocalBackend;
use rustaria_network::remote::RemoteBackend;
use rustaria_network::{
    EstablishingInstance, NetworkBackend, NetworkInterface, Packet, Token,
};

use crate::network::join::{PlayerJoinData, PlayerJoinInstance};
use crate::Server;

pub mod join;
pub mod packet;

pub struct Networking {
    local: Option<LocalBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>>,
    remote:
        Option<RemoteBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>>,
}

impl Networking {
    pub fn new(
        local: Option<LocalBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>>,
        remote: Option<
            RemoteBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>,
        >,
    ) -> Networking {
        Networking { local, remote }
    }

    pub fn tick(server: &mut Server) {
        let mut interface = ServerNetworkInterface {};
        if let Some(backend) = &mut server.network.local {
            backend.poll(&mut interface);
        }

        if let Some(backend) = &mut server.network.remote {
            backend.poll(&mut interface);
        }
    }

    pub fn send(&self, to: Token, packet: ServerPacket) -> Result<()> {
        if let Some(backend) = &self.local {
            backend.send(to, packet.clone())?;
        }

        if let Some(backend) = &self.remote {
            backend.send(to, packet)?;
        }

        Ok(())
    }

    pub fn distribute(&self, from: Token, packet: ServerPacket) -> Result<()> {
        if let Some(backend) = &self.local {
            backend.distribute(from, packet.clone())?;
        }

        if let Some(backend) = &self.remote {
            backend.distribute(from, packet)?;
        }

        Ok(())
    }

    pub fn get_local_mut(&mut self) -> &mut Option<LocalBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>> {
        &mut self.local
    }

    pub fn get_remote_mut(&mut self) -> &mut Option<RemoteBackend<ClientPacket, ServerPacket, PlayerJoinInstance, PlayerJoinData>> {
        &mut self.remote
    }
}

pub struct ServerNetworkInterface;

impl<I, O, C, EI> NetworkInterface<I, O, C, EI> for ServerNetworkInterface
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn receive(&mut self, from: Token, packet: I) {
        todo!()
    }

    fn disconnected(&mut self, client: Token) {
        todo!()
    }

    fn connected(&mut self, client: Token, connection_data: C) {
        todo!()
    }

    fn establishing(&mut self) -> EI {
        todo!()
    }
}
