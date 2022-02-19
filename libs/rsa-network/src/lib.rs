use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crossbeam::channel::Sender;
use laminar::{Config, Packet, Socket, SocketEvent};
use serde::Serialize;

pub mod packet;
pub mod server;
pub mod client;

pub type Channel = u8;

#[derive(Copy, Clone)]
pub struct PacketDescriptor {
    pub priority: PacketPriority,
    pub order: PacketOrder,
}

impl PacketDescriptor {
    pub fn to_packet(&self, addr: &SocketAddr, payload: Vec<u8>) -> Packet {
        match self.priority {
            PacketPriority::Unreliable => match self.order {
                PacketOrder::Unordered => Packet::unreliable(*addr, payload),
                PacketOrder::Ordered(_) => panic!("Cannot have Ordered Unreliable packets."),
                PacketOrder::Sequenced(stream_id) => {
                    Packet::unreliable_sequenced(*addr, payload, stream_id)
                }
            },
            PacketPriority::Reliable => match self.order {
                PacketOrder::Unordered => Packet::reliable_unordered(*addr, payload),
                PacketOrder::Ordered(stream_id) => {
                    Packet::reliable_ordered(*addr, payload, stream_id)
                }
                PacketOrder::Sequenced(stream_id) => {
                    Packet::reliable_sequenced(*addr, payload, stream_id)
                }
            },
        }
    }
}

#[derive(Copy, Clone)]
pub enum PacketPriority {
    Unreliable,
    Reliable,
}

#[derive(Copy, Clone)]
pub enum PacketOrder {
    Unordered,
    Ordered(Option<Channel>),
    Sequenced(Option<Channel>),
}

pub fn create_socket(self_address: SocketAddr) -> Socket {
    Socket::bind_with_config(
        self_address,
        Config {
            idle_connection_timeout: Duration::from_secs(60),
            heartbeat_interval: Some(Duration::from_secs(10)),
            ..Config::default()
        },
    )
        .unwrap()
}

pub fn poll_once(socket: &mut Socket) -> SocketEvent {
    socket.manual_poll(Instant::now());
    let mut thing = socket.recv();
    while thing.is_none() {
        std::thread::sleep(Duration::from_millis(10));
        socket.manual_poll(Instant::now());
        thing = socket.recv();
    }

    thing.unwrap()
}

pub fn poll_packet(socket: &mut Socket) -> Option<Vec<u8>> {
    if let SocketEvent::Packet(packet) = poll_once(socket) {
        return Some(packet.payload().to_owned());
    }
    None
}

pub fn send_obj<D: Serialize>(
    socket: Sender<Packet>,
    addr: SocketAddr,
    data: &D,
) -> eyre::Result<()> {
    socket.send(Packet::reliable_unordered(addr, bincode::serialize(data)?))?;
    Ok(())
}
