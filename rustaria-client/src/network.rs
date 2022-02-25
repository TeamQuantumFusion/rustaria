use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::{bail, eyre};
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, warn};

use rustaria::api::Rustaria;
use rustaria::api::RustariaHash;
use rustaria::network::packet::{ClientPacket, ModListPacket, ServerPacket};
use rustaria::network::server::ServerNetwork;
use rustaria::network::{create_socket, poll_once, poll_packet, PacketDescriptor};
use rustaria::world::World;
use rustaria::{Server, KERNEL_VERSION};

// Client
pub trait ServerCom {
    fn tick(&mut self, rsa: &Rustaria) -> eyre::Result<()>;
    fn send(&mut self, packet: ClientPacket, desc: PacketDescriptor) -> eyre::Result<()>;
    fn receive(&mut self) -> Vec<ServerPacket>;
}

pub enum ConnectionError {
    InvalidHandshake,
    DifferentServerKernelVersion((u8, u8, u8)),
}

// Server Com Implementations
pub struct RemoteServer {
    socket: Socket,
    server_addr: SocketAddr,
    shutdown: bool,
}

impl RemoteServer {
    pub fn new(
        api: &Rustaria,
        server_addr: SocketAddr,
        self_address: SocketAddr,
    ) -> eyre::Result<RemoteServer> {
        let mut socket = create_socket(self_address);

        debug!("{server_addr} RC: Connecting");
        socket.send(Packet::reliable_unordered(server_addr, vec![69]))?;

        match poll_once(&mut socket) {
            SocketEvent::Connect(_) => {}
            _ => bail!("Invalid Handshake order"),
        }

        // Check kernel version
        debug!("{server_addr} RC: Checking Kernel Version");
        let packet = poll_packet(&mut socket).ok_or_else(|| eyre!("Invalid Handshake order"))?;
        let server_version = (packet[0], packet[1], packet[2]);
        if server_version != KERNEL_VERSION {
            socket.send(Packet::reliable_unordered(server_addr, vec![0]))?;
            bail!("Server uses kernel {server_version:?} while client uses {KERNEL_VERSION:?}");
        }

        // Proceed
        debug!("{server_addr} RC: Continue");
        socket.send(Packet::reliable_unordered(server_addr, vec![1]))?;

        // get sha256
        debug!("{server_addr} RC: Checking Rustaria Hash");
        let hash = RustariaHash::parse(
            poll_packet(&mut socket).ok_or_else(|| eyre!("Could not get RegistryHash"))?,
        );

        if hash != api.hash {
            // send modlist
            socket.send(Packet::reliable_unordered(server_addr, vec![1]))?;

            let pkt = poll_packet(&mut socket).ok_or_else(|| eyre!("Could not get ModList"))?;
            let mod_list: ModListPacket = bincode::deserialize(&pkt)?;

            for (mod_name, mod_version) in mod_list.mod_list.into_iter() {
                if let Some(version) = api.mod_list.get(&mod_name) {
                    if version != &mod_version {
                        warn!(
                            "Invalid version. [{mod_name}]. Remote: {mod_version}, Local: {version}"
                        );
                    }
                }
                warn!("Missing mod [{mod_name}] v{mod_version}")
            }

            bail!("Invalid mods");
        } else {
            socket.send(Packet::reliable_unordered(server_addr, vec![0]))?;
        }

        debug!("{server_addr} RC: Connected");
        Ok(RemoteServer {
            socket,
            server_addr,
            shutdown: false,
        })
    }
}

impl ServerCom for RemoteServer {
    fn tick(&mut self, _: &Rustaria) -> eyre::Result<()> {
        self.socket.manual_poll(Instant::now());
        Ok(())
    }

    fn send(&mut self, packet: ClientPacket, desc: PacketDescriptor) -> eyre::Result<()> {
        debug!("Sending {packet:?}");
        self.socket
            .send(desc.to_packet(&self.server_addr, bincode::serialize(&packet)?))?;
        Ok(())
    }

    fn receive(&mut self) -> Vec<ServerPacket> {
        let mut out = Vec::new();
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.server_addr {
                        if let Ok(packet) = bincode::deserialize(packet.payload()) {
                            debug!("Received {packet:?}");
                            out.push(packet);
                        }
                    } else {
                        debug!("UNKNOWN PACKET");
                    }
                }
                SocketEvent::Connect(_) => {}
                SocketEvent::Timeout(_) => {}
                SocketEvent::Disconnect(addr) => {
                    if addr == self.server_addr {
                        self.shutdown = true;
                        return Vec::new();
                    }
                }
            }
        }

        out
    }
}

pub struct IntegratedServer {
    to_server: Sender<ClientPacket>,
    from_server: Receiver<ServerPacket>,
    server: Server,
}

impl IntegratedServer {
    pub fn new(world: World, remote: Option<SocketAddr>) -> IntegratedServer {
        let (to_client, from_server) = crossbeam::channel::unbounded();
        let (to_server, from_client) = crossbeam::channel::unbounded();
        // todo dont unwrap

        let mut server = Server::new(world, ServerNetwork::new(remote, true));
        server.network.join_local(to_client, from_client).unwrap();
        IntegratedServer {
            to_server,
            from_server,
            server,
        }
    }
}

impl ServerCom for IntegratedServer {
    fn tick(&mut self, rustaria: &Rustaria) -> eyre::Result<()> {
        self.server.tick(rustaria)
    }

    fn send(&mut self, packet: ClientPacket, _desc: PacketDescriptor) -> eyre::Result<()> {
        debug!("Sending {:?}", packet);
        self.to_server.send(packet)?;
        Ok(())
    }

    fn receive(&mut self) -> Vec<ServerPacket> {
        let mut out = Vec::new();
        while let Ok(packet) = self.from_server.try_recv() {
            debug!("Received {:?}", packet);
            out.push(packet);
        }
        out
    }
}
