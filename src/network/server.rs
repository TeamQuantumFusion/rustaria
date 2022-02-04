use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};

use crate::network::{create_socket};
use crate::network::packet::{ClientPacket, ServerPacket};

pub trait ClientCom {
    fn tick(&mut self);
    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> ;
    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()> ;
    fn receive(&mut self) -> Vec<(Token, ClientPacket)>;
}

pub trait LocalPlayerJoin {
    fn join(&mut self, send: Sender<ServerPacket>, receiver: Receiver<ClientPacket>) -> u8;
}

pub struct Server<C: ClientCom> {
    pub network: C,
}

impl<C: ClientCom> Server<C> {
    pub fn tick(&mut self) {
        self.network.tick();
        for (token, packet) in self.network.receive() {
            println!("SERVER: Received \"{:?}\" from {:?}", packet, token);
            self.network.send(&token, &ServerPacket::FuckOff).unwrap();
            //self.network.distribute(&token, &packet);
        }
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

impl LocalClientCom {
    fn new() -> LocalClientCom {
        LocalClientCom {
            local_id: 0,
            local_players: Default::default(),
        }
    }
}

impl LocalPlayerJoin for LocalClientCom {
    fn join(&mut self, send: Sender<ServerPacket>, receiver: Receiver<ClientPacket>) -> u8 {
        let id = self.local_id;
        self.local_players.insert(id, (send, receiver));
        self.local_id += 1;
        id
    }
}

impl ClientCom for LocalClientCom {
    fn tick(&mut self) {
        // beg
    }

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        for (id, (to_client, _)) in &self.local_players {
            if Token::Local(*id) != *source {
                to_client.send((*packet).clone())?;
            }
        }

        Ok(())
    }

    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()>{
        if let Token::Local(id) = target {
            if let Some((sender, _)) = self.local_players.get(id) {
                sender.send((*packet).clone())?;
            }
        }

        Ok(())
    }

    fn receive(&mut self) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        for (id, (_, receiver)) in &self.local_players {
            while let Ok(packet) = receiver.try_recv() {
                out.push((Token::Local(*id), packet))
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

impl RemoteClientCom {
    fn new(addr: SocketAddr) -> RemoteClientCom {
        RemoteClientCom {
            socket: create_socket(addr),
            remote_players: Default::default(),
        }
    }
}

impl ClientCom for RemoteClientCom {
    fn tick(&mut self) {
        self.socket.manual_poll(Instant::now());
    }

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()>  {
        for addr in &self.remote_players {
            if Token::Remote(*addr) != *source {
                self.socket
                    .send(Packet::reliable_unordered(*addr, bincode::serialize(packet)?)).unwrap();
            }
        }

        Ok(())
    }

    fn send(&mut self, target: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        if let Token::Remote(addr) = target {
            self.socket
                .send(Packet::reliable_unordered(*addr, bincode::serialize(packet)?)).unwrap();
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
                        out.push((
                            Token::Remote(addr),
                            client_packet,
                        ));
                    }
                }
                SocketEvent::Connect(address) => {
                    println!("SERVER: New Client connected {}", address);
                    self.remote_players.insert(address);
                }
                SocketEvent::Timeout(address) => {
                    println!("SERVER: Client timeout {}", address);
                }
                SocketEvent::Disconnect(address) => {
                    println!("SERVER: Client disconnected {}", address);
                    self.remote_players.remove(&address);
                }
            }
        }

        out
    }
}

// Multi player
pub struct MultiClientCom {
    local: LocalClientCom,
    remote: RemoteClientCom,
}

impl LocalPlayerJoin for MultiClientCom {
    fn join(&mut self, send: Sender<ServerPacket>, receiver: Receiver<ClientPacket>) -> u8 {
        self.local.join(send, receiver)
    }
}

impl ClientCom for MultiClientCom {
    fn tick(&mut self) {
        self.local.tick();
        self.remote.tick();
    }

    fn distribute(&mut self, source: &Token, packet: &ServerPacket) -> eyre::Result<()> {
        self.local.distribute(source, packet)?;
        self.remote.distribute(source, packet)?;
        Ok(())
    }

    fn send(&mut self, target: &Token, packet: &ServerPacket)-> eyre::Result<()>  {
        self.local.send(target, packet)?;
        self.remote.send(target, packet)?;
        Ok(())
    }

    fn receive(&mut self) -> Vec<(Token, ClientPacket)> {
        let mut out = Vec::new();
        for x in self.local.receive() {
            out.push(x);
        }
        for x in self.remote.receive() {
            out.push(x);
        }

        out
    }
}

pub enum ServerInfo {
    Local,
    Remote(SocketAddr),
    LocalRemote(SocketAddr),
}

pub fn new_singleplayer_server() -> Server<LocalClientCom> {
    Server {
        network: LocalClientCom::new(),
    }
}

pub fn new_multiplayer_server(addr: SocketAddr) -> Server<MultiClientCom> {
    Server {
        network: MultiClientCom {
            local: LocalClientCom::new(),
            remote: RemoteClientCom::new(addr),
        },
    }
}

pub fn new_dedicated_server(addr: SocketAddr) -> Server<RemoteClientCom> {
    Server {
        network: RemoteClientCom::new(addr),
    }
}
