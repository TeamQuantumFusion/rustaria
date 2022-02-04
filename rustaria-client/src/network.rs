use std::net::SocketAddr;
use std::time::Instant;

use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use rustaria::network::{create_socket, poll_once};
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::network::server::{ClientCom, LocalPlayerJoin, Server};


// Client
pub trait ServerCom {
    fn tick(&mut self);
    fn send(&mut self, packet: &ClientPacket) -> eyre::Result<()>;
    fn receive(&mut self) -> Vec<ServerPacket>;
}

pub struct Client<C: ServerCom> {
    pub network: C,
}

impl<C: ServerCom> Client<C> {
    pub fn tick(&mut self) {
        self.network.tick();

        for packet in self.network.receive() {
            println!("CLIENT: Received \"{:?}\"", packet);
        }
    }

    pub fn send(&mut self, packet: &ClientPacket) {
        println!("CLIENT: Sending \"{:?}\"", packet);
        self.network.send(packet);
    }
}

// Server Com Implementations
pub struct RemoteServerCom {
    socket: Socket,
    server_addr: SocketAddr,
    shutdown: bool,
}

impl RemoteServerCom {
    pub fn new(server_addr: SocketAddr, self_address: SocketAddr) -> RemoteServerCom {
        let mut socket = create_socket(self_address);
        socket.send(Packet::reliable_unordered(server_addr, vec![69])).unwrap();
        if let SocketEvent::Connect(connect) = poll_once(&mut socket) {
            if let SocketEvent::Packet(handshake) = poll_once(&mut socket) {
                if handshake.payload().eq(&[69]) {
                    println!("Connected to {}. nice", connect);
                } else {
                    println!("Err(ClientError::InvalidHandshakeCode);")
                }
            } else {
                println!("Err(ClientError::InvalidHandshakeOrder);")
            }
        } else {
            println!("Err(ClientError::InvalidHandshakeOrder);")
        }


        RemoteServerCom {
            socket,
            server_addr,
            shutdown: false,
        }
    }
}

impl ServerCom for RemoteServerCom {
    fn tick(&mut self) {
        self.socket.manual_poll(Instant::now());
    }

    fn send(&mut self, packet: &ClientPacket) -> eyre::Result<()> {
        self.socket.send(Packet::reliable_unordered(self.server_addr, bincode::serialize(packet)?))?;
        Ok(())
    }

    fn receive(&mut self) -> Vec<ServerPacket> {
        let mut out = Vec::new();
        while let Some(packet) = self.socket.recv() {
            match packet {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.server_addr {
                        if let Ok(packet) = bincode::deserialize(packet.payload()) {
                            out.push(packet);
                        }
                    } else {
                        println!("Warning unknown packet");
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
    pub fn new<LPJ: LocalPlayerJoin + ClientCom>(server: &mut Server<LPJ>) -> LocalServerCom {
        let (to_client, from_server) = crossbeam::channel::unbounded();
        let (to_server, from_client) = crossbeam::channel::unbounded();
        server.network.join(to_client, from_client);
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

    fn send(&mut self, packet: &ClientPacket) -> eyre::Result<()> {
        self.to_server.send((*packet).clone());

        Ok(())
    }

    fn receive(&mut self) -> Vec<ServerPacket> {
        let mut out = Vec::new();
        while let Ok(packet) = self.from_server.try_recv() {
            out.push(packet);
        }
        out
    }
}