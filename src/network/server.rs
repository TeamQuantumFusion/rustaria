use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::ContextCompat;
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, info, warn};

use crate::api::Rustaria;
use crate::network::packet::{ClientPacket, ModListPacket, ServerPacket};
use crate::network::{create_socket, send_obj, PacketDescriptor};
use crate::KERNEL_VERSION;

pub trait ClientCom {
    fn tick(&mut self);
    fn distribute(
        &mut self,
        source: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()>;
    fn send(
        &mut self,
        target: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()>;
    fn receive(&mut self, rustaria: &Rustaria) -> Vec<(Token, ClientPacket)>;
}

pub struct ServerNetwork {
    local_com: Option<Box<LocalClientCom>>,
    remote_com: Option<Box<RemoteClientCom>>,
}

impl ServerNetwork {
    pub fn new(remote: Option<SocketAddr>, local: bool) -> ServerNetwork {
        ServerNetwork {
            local_com: local.then(|| {
                Box::new(LocalClientCom {
                    local_id: 0,
                    local_players: Default::default(),
                })
            }),
            remote_com: remote.map(|addr| {
                Box::new(RemoteClientCom {
                    socket: create_socket(addr),
                    remote_players: Default::default(),
                })
            }),
        }
    }

    pub fn tick(&mut self) {
        if let Some(com) = self.local_com.as_mut() {
            com.tick()
        }
        if let Some(com) = self.remote_com.as_mut() {
            com.tick()
        }
    }

    pub fn join_local(
        &mut self,
        send: Sender<ServerPacket>,
        receiver: Receiver<ClientPacket>,
    ) -> eyre::Result<u8> {
        let x = self
            .local_com
            .as_mut()
            .wrap_err(ServerError::DedicatedServerDoesNotSupportLocalPlayer)?;
        let id = x.local_id;
        x.local_id += 1;
        x.local_players.insert(id, (send, receiver));
        Ok(id)
    }

    pub fn distribute(
        &mut self,
        source: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        if let Some(com) = &mut self.local_com {
            com.distribute(source, packet, desc)?;
        }
        if let Some(com) = &mut self.remote_com {
            com.distribute(source, packet, desc)?;
        }
        Ok(())
    }

    pub fn send(
        &mut self,
        token: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        if let Some(com) = &mut self.local_com {
            com.send(token, packet, desc)?;
        }
        if let Some(com) = &mut self.remote_com {
            com.send(token, packet, desc)?;
        }
        Ok(())
    }

    pub fn receive(&mut self, rustaria: &Rustaria) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        if let Some(com) = &mut self.local_com {
            for x in com.receive(rustaria) {
                out.push(x);
            }
        }

        if let Some(com) = &mut self.remote_com {
            for x in com.receive(rustaria) {
                out.push(x);
            }
        }

        out
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Token {
    Server,
    Local(u8),
    Remote(SocketAddr),
}

// Network implementations
// Single player
pub struct LocalClientCom {
    local_id: u8,
    local_players: HashMap<u8, (Sender<ServerPacket>, Receiver<ClientPacket>)>,
}

impl ClientCom for LocalClientCom {
    fn tick(&mut self) {
        // beg
    }

    fn distribute(
        &mut self,
        source: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        debug!("Distributing {packet:?} from {source:?}");

        for (id, (to_client, _)) in &self.local_players {
            if Token::Local(*id) != *source {
                to_client.send(packet.clone())?;
            }
        }

        Ok(())
    }

    fn send(
        &mut self,
        target: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        if let Token::Local(id) = target {
            if let Some((sender, _)) = self.local_players.get(id) {
                debug!("Sending {packet:?} to {target:?}");
                sender.send(packet.clone())?;
            }
        }

        Ok(())
    }

    fn receive(&mut self, _: &Rustaria) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        for (id, (_, receiver)) in &self.local_players {
            while let Ok(packet) = receiver.try_recv() {
                let token = Token::Local(*id);
                debug!("Received {packet:?} from {token:?}");
                out.push((token, packet))
            }
        }

        out
    }
}

// Dedicated Server
pub struct RemoteClientCom {
    socket: Socket,
    remote_players: HashMap<SocketAddr, ClientConnection>,
}

impl ClientCom for RemoteClientCom {
    fn tick(&mut self) {
        self.socket.manual_poll(Instant::now());
    }

    fn distribute(
        &mut self,
        source: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        debug!("Distributing {packet:?} from {source:?}");
        for (addr, connection) in &self.remote_players {
            if let ClientConnection::Playing = connection {
                if Token::Remote(*addr) != *source {
                    let payload = bincode::serialize(packet)?;
                    self.socket.send(desc.to_packet(addr, payload)).unwrap();
                }
            }
        }

        Ok(())
    }

    fn send(
        &mut self,
        target: &Token,
        packet: &ServerPacket,
        desc: PacketDescriptor,
    ) -> eyre::Result<()> {
        if let Token::Remote(addr) = target {
            // todo i dont think we should unwrap or return on serialize fail. this may just be a rouge client.
            let data = bincode::serialize(packet)?;
            debug!(
                "Sending {} to {:?}. {}B",
                packet.get_type_str(),
                target,
                data.len()
            );

            self.socket.send(desc.to_packet(addr, data)).unwrap();
        }

        Ok(())
    }

    fn receive(&mut self, rustaria: &Rustaria) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    // Handshake
                    let addr = packet.addr();
                    let connection = self.remote_players.get_mut(&addr);
                    if let Some(ClientConnection::Playing) = connection {
                        let token = Token::Remote(addr);
                        let c_packet: bincode::Result<ClientPacket> =
                            bincode::deserialize(packet.payload());
                        if let Ok(client_packet) = c_packet {
                            debug!("Received {client_packet:?} from {token:?}");
                            out.push((token, client_packet));
                        } else {
                            warn!("Unknown Packet from {token:?}");
                        }
                    } else if let Some(ClientConnection::Handshaking(hand)) = connection {
                        hand.handle(rustaria, addr, &packet, &self.socket.get_packet_sender());
                        match &hand {
                            HandshakingStep::Failed => {
                                self.remote_players.remove(&addr);
                            }
                            HandshakingStep::Joined => {
                                self.remote_players.insert(addr, ClientConnection::Playing);
                            }
                            _ => {}
                        };
                    } else if packet.payload() == [69] {
                        info!("New shit");
                        HandshakingStep::KernelSend.handle(
                            rustaria,
                            addr,
                            &packet,
                            &self.socket.get_packet_sender(),
                        );
                        self.remote_players.insert(
                            addr,
                            ClientConnection::Handshaking(HandshakingStep::KernelProceedAwait),
                        );
                    }
                }
                SocketEvent::Connect(_) => {
                    // kinda irrelevant.
                }
                SocketEvent::Timeout(addr) => {
                    info!("Client Timed out {addr}");
                }
                SocketEvent::Disconnect(addr) => {
                    info!("Client Disconnected {addr}");
                    self.remote_players.remove(&addr);
                }
            }
        }

        out
    }
}

pub enum ClientConnection {
    Handshaking(HandshakingStep),
    Playing,
}

pub enum HandshakingStep {
    KernelSend,
    KernelProceedAwait,
    RegistryMatchAwait,
    Failed,
    Joined,
}

impl HandshakingStep {
    pub fn handle(
        &mut self,
        rustaria: &Rustaria,
        addr: SocketAddr,
        packet: &Packet,
        sender: &Sender<Packet>,
    ) {
        match self {
            HandshakingStep::KernelSend => {
                debug!("HS {addr}: Sending Kernel Version");
                if let Err(error) = send_obj(&sender, addr, &KERNEL_VERSION) {
                    warn!("Could not send kernel version {error}");
                    *self = HandshakingStep::Failed;
                } else {
                    *self = HandshakingStep::KernelProceedAwait;
                }
            }
            HandshakingStep::KernelProceedAwait => {
                if packet.payload() == [1] {
                    debug!("HS {addr}: Sending Rustaria Hash");
                    if let Err(error) = send_obj(&sender, addr, &rustaria.hash) {
                        warn!("Could not send Rustaria Hash {error}");
                        *self = HandshakingStep::Failed;
                    } else {
                        *self = HandshakingStep::RegistryMatchAwait;
                    }
                } else {
                    *self = HandshakingStep::Failed;
                }
            }
            HandshakingStep::RegistryMatchAwait => {
                if packet.payload() == [1] {
                    debug!("HS {addr}: Sending Modlist");
                    if let Err(error) = send_obj(&sender, addr, &ModListPacket::new(rustaria)) {
                        warn!("Could not send modlist {error}");
                    }
                    *self = HandshakingStep::Failed;
                } else if packet.payload() == [0] {
                    debug!("HS {addr}: Player Connected");
                    *self = HandshakingStep::Joined;
                } else {
                    *self = HandshakingStep::Failed;
                }
            }
            _ => {}
        }
    }
}

pub enum ServerError {
    InvalidIp,
    DedicatedServerDoesNotSupportLocalPlayer,
}

impl ServerError {
    pub fn as_str(&self) -> &str {
        match self {
            ServerError::InvalidIp => "Invalid ip address.",
            ServerError::DedicatedServerDoesNotSupportLocalPlayer => {
                "Tried to join a local player on a dedicated server."
            }
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
