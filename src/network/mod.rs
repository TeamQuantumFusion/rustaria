use std::net::SocketAddr;
use std::time::{Duration, Instant};

use laminar::{Config, Socket, SocketEvent};

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
    let mut thing = socket.recv();
    while thing.is_none() {
        std::thread::sleep(Duration::from_millis(10));
        socket.manual_poll(Instant::now());
        thing = socket.recv();
    }

    thing.unwrap()
}