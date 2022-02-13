use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::{bail, eyre};
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, warn};

use rustaria::api::Rustaria;
use rustaria::api::RustariaHash;
use rustaria::network::packet::{ClientPacket, ModListPacket, ServerPacket};
use rustaria::network::{create_socket, poll_once, poll_packet, PacketDescriptor};
use rustaria::{Server, KERNEL_VERSION};

// Client
pub trait ServerCom {
    fn tick(&mut self);
    fn send(&mut self, packet: &ClientPacket, desc: PacketDescriptor) -> eyre::Result<()>;
    fn receive(&mut self) -> Vec<ServerPacket>;
}

pub struct Client<C: ServerCom> {
    pub network: C,
}

impl<C: ServerCom> Client<C> {}

pub enum ConnectionError {
    InvalidHandshake,
    DifferentServerKernelVersion((u8, u8, u8)),
}

// Server Com Implementations
pub struct RemoteServerCom {
    socket: Socket,
    server_addr: SocketAddr,
    shutdown: bool,
}

impl RemoteServerCom {
    pub fn new(
        rustaria: &Rustaria,
        server_addr: SocketAddr,
        self_address: SocketAddr,
    ) -> eyre::Result<RemoteServerCom> {
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

        if hash != rustaria.hash {
            // send modlist
            socket.send(Packet::reliable_unordered(server_addr, vec![1]))?;

            let pkt = poll_packet(&mut socket).ok_or_else(|| eyre!("Could not get ModList"))?;
            let mod_list: ModListPacket = bincode::deserialize(&pkt)?;

            for (mod_name, mod_version) in mod_list.data.into_iter() {
                if let Some(plugin) = rustaria.plugins.get(&mod_name) {
                    let local_version = &plugin.manifest.version;
                    if local_version != &mod_version {
                        warn!(
                            "Invalid version. [{mod_name}]. Remote: {mod_version}, Local: {local_version}"
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
        Ok(RemoteServerCom {
            socket,
            server_addr,
            shutdown: false,
        })
    }
}

impl ServerCom for RemoteServerCom {
    fn tick(&mut self) {
        self.socket.manual_poll(Instant::now());
    }

    fn send(&mut self, packet: &ClientPacket, desc: PacketDescriptor) -> eyre::Result<()> {
        debug!("Sending {packet:?}");
        self.socket
            .send(desc.to_packet(&self.server_addr, bincode::serialize(packet)?))?;
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

pub struct LocalServerCom {
    to_server: Sender<ClientPacket>,
    from_server: Receiver<ServerPacket>,
}

impl LocalServerCom {
    pub fn new(server: &mut Server) -> LocalServerCom {
        let (to_client, from_server) = crossbeam::channel::unbounded();
        let (to_server, from_client) = crossbeam::channel::unbounded();
        // todo dont unwrap
        server.network.join_local(to_client, from_client).unwrap();
        LocalServerCom {
            to_server,
            from_server,
        }
    }
}

impl ServerCom for LocalServerCom {
    fn tick(&mut self) {
        // beg
    }

    fn send(&mut self, packet: &ClientPacket, _desc: PacketDescriptor) -> eyre::Result<()> {
        debug!("Sending {:?}", packet);
        self.to_server.send((*packet).clone())?;
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
