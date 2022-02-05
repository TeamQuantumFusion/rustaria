use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use eyre::ContextCompat;
use laminar::{Packet, Socket, SocketEvent};
use tracing::{debug, info};

use crate::network::create_socket;
use crate::network::packet::{ClientPacket, ServerPacket};

pub trait ClientCom {
    fn tick(&mut self);
    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()>;
    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()>;
    fn receive(&mut self) -> Vec<(Token, ClientPacket)>;
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
            remote_com: remote.map(|addr| Box::new(RemoteClientCom {
                socket: create_socket(addr),
                remote_players: Default::default(),
            })),
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

    pub fn join_local(&mut self, send: Sender<ServerPacket>, receiver: Receiver<ClientPacket>) -> eyre::Result<u8> {
        let x = self.local_com.as_mut().wrap_err(ServerError::DedicatedServerDoesNotSupportLocalPlayer)?;
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

    pub fn receive(&mut self) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        if let Some(com) = &mut self.local_com {
            for x in com.receive() {
                out.push(x);
            }
        }

        if let Some(com) = &mut self.remote_com {
            for x in com.receive() {
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

    fn receive(&mut self) -> Vec<(Token, ClientPacket)> {
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
    remote_players: HashSet<SocketAddr>,
}

impl ClientCom for RemoteClientCom {
    fn tick(&mut self) {
        self.socket.manual_poll(Instant::now());
    }

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        debug!("Distributing {:?} from {:?}", packet, source);
        for addr in &self.remote_players {
            if Token::Remote(*addr) != *source {
                self.socket
                    .send(Packet::reliable_unordered(
                        *addr,
                        bincode::serialize(packet)?,
                    ))
                    .unwrap();
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

    fn receive(&mut self) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    // Handshake
                    let addr = packet.addr();
                    if !self.remote_players.contains(&addr) {
                        self.socket
                            .send(Packet::reliable_unordered(addr, vec![69]))
                            .unwrap();
                    } else if let Ok(client_packet) = bincode::deserialize(packet.payload()) {
                        let token = Token::Remote(addr);
                        debug!("Received {:?} from {:?}", client_packet, token);
                        out.push((token, client_packet));
                    }
                }
                SocketEvent::Connect(address) => {
                    info!("Client Connected {}", address);
                    self.remote_players.insert(address);
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

pub enum ServerError {
    InvalidIp,
    DedicatedServerDoesNotSupportLocalPlayer,
}

impl ServerError {
    pub fn as_str(&self) -> &str {
        match self {
            ServerError::InvalidIp => "Invalid ip address.",
            ServerError::DedicatedServerDoesNotSupportLocalPlayer => "Tried to join a local player on a dedicated server."
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}