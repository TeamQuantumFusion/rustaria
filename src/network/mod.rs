use std::net::SocketAddr;
use std::time::{Duration, Instant};

use laminar::{Config, Packet, Socket, SocketEvent};
use serde::Serialize;
use crate::blake3::OUT_LEN;

pub mod server;
pub mod packet;

pub enum PacketPriority {
    Unreliable,
    Reliable,
}

pub enum PacketOrder {
    None,
    Channel(u8),
}

pub fn create_socket(self_address: SocketAddr) -> Socket {
    Socket::bind_with_config(self_address, Config {
        idle_connection_timeout: Duration::from_secs(60),
        heartbeat_interval: Some(Duration::from_secs(10)),
        ..Config::default()
    }).unwrap()
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

pub fn send_obj<D: Serialize>(socket: &mut Socket, addr: SocketAddr, data: &D) -> eyre::Result<()>{
    socket.send(Packet::reliable_unordered(addr, bincode::serialize(data)?));
    Ok(())
}