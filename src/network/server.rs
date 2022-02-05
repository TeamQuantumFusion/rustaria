use std::collections::{HashMap};
use std::fmt::Formatter;
use std::fmt::Display;
use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::{ContextCompat, eyre};
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, info, warn};

use crate::api::{Rustaria, RustariaHash};
use crate::KERNEL_VERSION;
use crate::network::{create_socket, send_obj};
use crate::network::packet::{ClientPacket, ModListPacket, ServerPacket};

pub trait ClientCom {
    fn tick(&mut self);
    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()>;
    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()>;
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
                }
                )
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

    pub fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        if let Some(com) = &mut self.local_com {
            com.distribute(source, packet)?;
        }
        if let Some(com) = &mut self.remote_com {
            com.distribute(source, packet)?;
        }
        Ok(())
    }

    pub fn send(&mut self, token: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        if let Some(com) = &mut self.local_com {
            com.send(token, packet)?;
        }
        if let Some(com) = &mut self.remote_com {
            com.send(token, packet)?;
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

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        debug!("Distributing {:?} from {:?}", packet, source);

        for (id, (to_client, _)) in &self.local_players {
            if Token::Local(*id) != *source {
                to_client.send((*packet).clone())?;
            }
        }

        Ok(())
    }

    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        if let Token::Local(id) = target {
            if let Some((sender, _)) = self.local_players.get(id) {
                debug!("Sending {:?} to {:?}", packet, target);
                sender.send((*packet).clone())?;
            }
        }

        Ok(())
    }

    fn receive(&mut self, rustaria: &Rustaria) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        for (id, (_, receiver)) in &self.local_players {
            while let Ok(packet) = receiver.try_recv() {
                let token = Token::Local(*id);
                debug!("Received {:?} from {:?}", packet, token);
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

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        debug!("Distributing {:?} from {:?}", packet, source);
        for (addr, connection) in &self.remote_players {
            if let ClientConnection::Playing = connection {
                if Token::Remote(*addr) != *source {
                    self.socket
                        .send(Packet::reliable_unordered(
                            *addr,
                            bincode::serialize(packet)?,
                        ))
                        .unwrap();
                }
            }
        }

        Ok(())
    }

    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        if let Token::Remote(addr) = target {
            debug!("Sending {:?} to {:?}", packet, target);
            self.socket
                .send(Packet::reliable_unordered(
                    *addr,
                    bincode::serialize(packet)?,
                ))
                .unwrap();
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
                            debug!("Received {:?} from {:?}", client_packet, token);
                            out.push((token, client_packet));
                        } else {
                            warn!("Unknown Packet from {:?}", token);
                        }
                    } else if let Some(ClientConnection::Handshaking(hand)) = connection {
                        match hand {
                            HandshakingStep::KernelProceedAwait => {
                                if packet.payload() == [1] {
                                    debug!("HS {}: Sending Rustaria Hash", addr);
                                    send_obj(&mut self.socket, addr, &rustaria.hash);
                                    *hand = HandshakingStep::RegistryMatchAwait;
                                } else {
                                    self.remote_players.remove(&addr);
                                }
                            }
                            HandshakingStep::RegistryMatchAwait => {
                                if packet.payload() == [1] {
                                    debug!("HS {}: Sending Modlist", addr);
                                    send_obj(&mut self.socket, addr, &ModListPacket::new(rustaria));
                                    self.remote_players.remove(&addr);
                                } else if packet.payload() == [0] {
                                    debug!("HS {}: Player Connected", addr);
                                    self.remote_players.insert(addr, ClientConnection::Playing);
                                } else {
                                    self.remote_players.remove(&addr);
                                }
                            }
                        }
                    } else if packet.payload() == &[69] {
                        send_obj(&mut self.socket, addr, &KERNEL_VERSION);
                        self.remote_players
                            .insert(addr, ClientConnection::Handshaking(HandshakingStep::KernelProceedAwait));
                    }
                }
                SocketEvent::Connect(address) => {
                    // kinda irrelevant.
                }
                SocketEvent::Timeout(address) => {
                    info!("Client Timed out {}", address);
                }
                SocketEvent::Disconnect(address) => {
                    info!("Client Disconnected {}", address);
                    self.remote_players.remove(&address);
                }
            }
        }

        out
    }
}

impl RemoteClientCom {
    fn handshake(
        packet: &Packet,
        addr: SocketAddr,
        rec: Receiver<Packet>,
        packet_sender: Sender<Packet>,
        hash: RustariaHash,
    ) -> eyre::Result<()> {
        if packet.payload() == vec![69] {
            packet_sender.send(Packet::reliable_unordered(
                addr,
                bincode::serialize(&crate::KERNEL_VERSION)?,
            ));
        } else {
            return Err(eyre!("Wrong handshake byte."));
        }

        if rec.recv()?.payload() == vec![1] {
            packet_sender.send(Packet::reliable_unordered(addr, (&hash.data).to_vec()));
        } else {
            return Err(eyre!("Wrong handshake format."));
        }

        if rec.recv()?.payload() == vec![1] {
            // Send modlist and kill it
        }

        Ok(())
    }
}

pub enum ClientConnection {
    Handshaking(HandshakingStep),
    Playing,
}

pub enum HandshakingStep {
    KernelProceedAwait,
    RegistryMatchAwait,
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
