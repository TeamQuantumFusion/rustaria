use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::{Duration, Instant};

use bimap::BiMap;
use laminar::{Config, Socket, SocketEvent};
use rsa_core::api::carrier::Carrier;

use rsa_core::logging::error;
use rsa_core::settings::VERSION;

use crate::packet::Packet;
use crate::server::ServerTickData;
use crate::Token;

pub struct Remote<I: Packet, O: Packet> {
	version: Vec<u8>,
	hash: Vec<u8>,

	// Networking
	socket: Socket,
	clients: BiMap<Token, SocketAddr>,
	establishing: HashMap<SocketAddr, EstablishingStep>,

	_p: PhantomData<(O, I)>,
}

pub struct RemoteSettings {
	/// A forced address to bind to on the server.
	pub address: Option<Ipv4Addr>,
	/// The port to use on the address.
	/// - default: 42069
	pub port: Option<u16>,
	/// The timeout is the time required for the client to be disconnected without a response.
	/// - default: 60s
	pub timeout: Duration,
}

impl<I: Packet, O: Packet> Remote<I, O> {
	pub fn new(settings: RemoteSettings, carrier: &Carrier) -> crate::Result<Remote<I, O>> {
		let config = Config {
			heartbeat_interval: Some(Duration::from_secs(5)),
			idle_connection_timeout: settings.timeout,
			..Default::default()
		};

		let ip = settings.address.unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
		let port = settings.port.unwrap_or(42069);

		Ok(Remote {
			version: VERSION.as_bytes().to_vec(),
			hash: carrier.get_hash().to_vec(),
			socket: { Socket::bind_with_config(SocketAddrV4::new(ip, port), config)? },
			clients: Default::default(),
			establishing: Default::default(),
			_p: Default::default(),
		})
	}

	pub fn tick(&mut self, data: &mut ServerTickData<I>) -> crate::Result<()> {
		self.socket.manual_poll(Instant::now());
		while let Ok(event) = self.socket.get_event_receiver().try_recv() {
			match event {
				SocketEvent::Packet(packet) => {
					let addr = packet.addr();
					if let Some(from) = self.clients.get_by_right(&addr) {
						if let Ok(packet) = bincode::deserialize(packet.payload()) {
							data.received.push((*from, packet));
						} else {
							error!(target: "tick@networking.server", "Invalid packet from {}@{}", from, addr);
						}
					} else {
						match self.establishing.entry(addr) {
							Entry::Occupied(mut entry) => match entry.get_mut() {
								value @ EstablishingStep::SendVersion => {
									send_raw(&self.socket, addr, self.version.clone())?;
									*value = EstablishingStep::SendHash;
								}
								value @ EstablishingStep::SendHash => {
									if packet.payload() == self.version {
										send_raw(&self.socket, addr, vec![1])?;
										send_raw(&self.socket, addr, self.hash.clone())?;
										*value = EstablishingStep::AwaitHash;
									} else {
										send_raw(&self.socket, addr, vec![0])?;
										entry.remove();
									}
								}
								EstablishingStep::AwaitHash => {
									if packet.payload() == self.hash {
										send_raw(&self.socket, addr, vec![1])?;
										data.to_connect.push(Token::new_v4());
									} else {
										send_raw(&self.socket, addr, vec![0])?;
									}
									entry.remove();
								}
							},
							Entry::Vacant(entry) => {
								if packet.payload() == [0x69] {
									entry.insert(EstablishingStep::SendVersion);
								}
							}
						}
					}
				}
				SocketEvent::Disconnect(addr) => {
					if let Some((token, _)) = self.clients.remove_by_right(&addr) {
						data.to_disconnect.push(token);
					}
				}
				_ => {}
			}
		}

		Ok(())
	}

	pub fn send(&self, to: Token, packet: &O) -> crate::Result<()> {
		if let Some(remote) = self.clients.get_by_left(&to) {
			self.socket.get_packet_sender().send(
				packet
					.get_desc()
					.packet(*remote, bincode::serialize(packet)?),
			)?;
			return Ok(());
		}
		Ok(())
	}

	pub fn send_all(&self, packet: &O) -> crate::Result<()> {
		for client in self.clients.right_values() {
			self.socket.get_packet_sender().send(
				packet
					.get_desc()
					.packet(*client, bincode::serialize(packet)?),
			)?;
		}

		Ok(())
	}
}

fn send_raw(socket: &Socket, addr: SocketAddr, payload: Vec<u8>) -> crate::Result<()> {
	socket
		.get_packet_sender()
		.send(laminar::Packet::reliable_unordered(addr, payload))?;

	Ok(())
}

enum EstablishingStep {
	SendVersion,
	SendHash,
	AwaitHash,
}
