use std::{
	net::SocketAddr,
	thread::sleep,
	time::{Duration, Instant},
};

use laminar::{Packet, Socket, SocketEvent};
use serde::{de::DeserializeOwned, Serialize};
use rsa_core::err::audit::Audit;
use rsa_core::err::Result;

pub struct Connector {
	socket: Socket,
	addr: SocketAddr,
}

impl Connector {
	pub fn new(addr: SocketAddr) -> laminar::Result<Connector> {
		Socket::bind(addr).map(|socket| Connector { socket, addr })
	}

	pub fn send<V: Serialize>(&mut self, value: &V) -> Result<()> {
		self.socket.send(Packet::reliable_ordered(
			self.addr,
			bincode::serialize(value)?,
			None,
		))?;
		self.socket.manual_poll(Instant::now());
		Ok(())
	}

	pub fn receive<V: DeserializeOwned>(&mut self) -> Result<V> {
		loop {
			self.socket.manual_poll(Instant::now());
			if let Some(packet) = self.socket.recv() {
				match packet {
					SocketEvent::Packet(packet) => {
						return Ok(bincode::deserialize(packet.payload())?);
					}
					SocketEvent::Disconnect(_) => {
						return Err(Audit::new("Disconnected"));
					}
					_ => {}
				}
			}

			// not to kill the client lol.
			sleep(Duration::from_millis(50));
		}
	}

	pub fn done(self) -> (Socket, SocketAddr) { (self.socket, self.addr) }
}
