use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::{ContextCompat, eyre, Report};
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, info, warn};

use rustaria::{KERNEL_VERSION, Server};
use rustaria::api::Rustaria;
use rustaria::api::RustariaHash;
use rustaria::network::{create_socket, PacketDescriptor, poll_once, poll_packet};
use rustaria::network::packet::{ClientPacket, ModListPacket, ServerPacket};

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
    pub fn new(rustaria: &Rustaria, server_addr: SocketAddr, self_address: SocketAddr) -> eyre::Result<RemoteServerCom> {
        let mut socket = create_socket(self_address);

        debug!("{} RC: Connecting", server_addr);
        socket.send(Packet::reliable_unordered(server_addr, vec![69])).unwrap();

        if let SocketEvent::Connect(_) = poll_once(&mut socket) {} else {
            return Err(eyre!("Invalid Handshake order"));
        }

        // Check kernel version
        debug!("{} RC: Checking Kernel Version", server_addr);
        let packet = poll_packet(&mut socket).wrap_err(eyre!("Invalid Handshake order"))?;
        let server_version = (packet[0], packet[1], packet[2]);
        if server_version != rustaria::KERNEL_VERSION {
            socket.send(Packet::reliable_unordered(server_addr, vec![0])).unwrap();
            return Err(eyre!("Server uses kernel {:?} while client uses {:?}", server_version, rustaria::KERNEL_VERSION));
        }


        // Proceed
        debug!("{} RC: Continue", server_addr);
        socket.send(Packet::reliable_unordered(server_addr, vec![1])).unwrap();

        // get sha256
        debug!("{} RC: Checking Rustaria Hash", server_addr);
        let hash = RustariaHash::parse(poll_packet(&mut socket).wrap_err(eyre!("Could not get RegistryHash"))?);
        if hash != rustaria.hash {
            // send modlist
            socket.send(Packet::reliable_unordered(server_addr, vec![1])).unwrap();
            let mod_list: ModListPacket = bincode::deserialize(&*poll_packet(&mut socket).wrap_err(eyre!("Could not get ModList"))?)?;

            let mut report = Vec::new();
            for (mod_name, mod_version) in mod_list.data {
                if let Some(plugin) = rustaria.plugins.get(&mod_name) {
                    let string = &plugin.manifest.version;
                    if *string != mod_version  {
                        report.push(format!("Invalid version. [{mod_name}]. Remote: {}, Local: {}", mod_version, string))

                    }
                } else {
                    report.push(format!("Missing mod [{mod_name}] v{}", mod_version))
                }
            }

            for x in report {
                warn!("{}", x);
            }
            return Err(Report::msg("Invalid mods"));
        } else {
            socket.send(Packet::reliable_unordered(server_addr, vec![0])).unwrap();
        }

        debug!("{} RC: Connected", server_addr);
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
        debug!("Sending {:?}", packet);
        self.socket.send(desc.to_packet(&self.server_addr, bincode::serialize(packet)?))?;
        Ok(())
    }

    fn receive(&mut self) -> Vec<ServerPacket> {
        let mut out = Vec::new();
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.server_addr {
                        if let Ok(packet) = bincode::deserialize(packet.payload()) {
                            debug!("Received {:?}", packet);
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

    fn send(&mut self, packet: &ClientPacket, desc: PacketDescriptor) -> eyre::Result<()> {
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