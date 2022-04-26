use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::RwLock;
use std::time::Instant;

use bimap::BiMap;
use crossbeam::channel::{Receiver, Sender, unbounded};
use laminar::{Packet, Socket, SocketEvent};

use rustaria_util::logging::{debug, error, trace, warn};

use crate::{EstablishingInstance, EstablishingStatus, NetworkInterface, Result, Token};

pub struct ServerNetworking<I: crate::Packet, O: crate::Packet, J> {
	local_connections: HashMap<Token, (Sender<O>, Receiver<I>)>,

	socket: Option<Socket>,
	remote_clients: BiMap<Token, SocketAddr>,
	remote_establishing: HashMap<SocketAddr, Box<dyn EstablishingInstance<J>>>,
	disconnected: RwLock<HashSet<Token>>,

	new_players: RwLock<Vec<(Token, J)>>,
}

impl<I: crate::Packet, O: crate::Packet, J> ServerNetworking<I, O, J> {
	pub fn new(remote: Option<SocketAddr>) -> Result<ServerNetworking<I, O, J>> {
		Ok(ServerNetworking {
			local_connections: Default::default(),
			socket: {
				if let Some(addr) = remote {
					Some(Socket::bind(addr)?)
				} else {
					None
				}
			},
			remote_clients: Default::default(),
			remote_establishing: Default::default(),
			disconnected: Default::default(),
			new_players: Default::default(),
		})
	}

	pub fn send(&self, to: Token, packet: O) -> Result<()> {
		trace!(target: "misc@networking.server", "Sending packet {packet:?} to {to}");
		if let Some(socket) = &self.socket {
			if let Some(remote) = self.remote_clients.get_by_left(&to) {
				socket.get_packet_sender().send(Packet::reliable_unordered(
					*remote,
					bincode::serialize(&packet)?,
				))?;
				return Ok(());
			}
		}

		if let Some((sender, _)) = self.local_connections.get(&to) {
			sender.send(packet).unwrap();
		}
		Ok(())
	}

	pub fn send_others(&self, not: Token, packet: O) -> Result<()> {
		for to in self.remote_clients.left_values() {
			if *to != not {
				self.send(*to, packet.clone())?;
			}
		}

		for to in self.local_connections.keys() {
			if *to != not {
				self.send(*to, packet.clone())?;
			}
		}

		Ok(())
	}

	pub fn send_all(&self, packet: O) -> Result<()> {
		for to in self.remote_clients.left_values() {
			self.send(*to, packet.clone())?;
		}

		for to in self.local_connections.keys() {
			self.send(*to, packet.clone())?;
		}

		Ok(())
	}

	// TODO get rid of this interface, its really annoying because of the &mut that it requires as this instance is prob inside of the impl.
	pub fn poll(&mut self, interface: &mut impl NetworkInterface<I, O, J>) {
		for (client, connection_data) in self.new_players.write().unwrap().drain(..) {
			interface.connected(client, connection_data);
		}

		for (from, (_, receiver)) in &self.local_connections {
			while let Ok(packet) = receiver.try_recv() {
				interface.receive(*from, packet);
			}
		}

		if let Some(socket) = &mut self.socket {
			socket.manual_poll(Instant::now());
			while let Ok(event) = socket.get_event_receiver().try_recv() {
				match event {
					SocketEvent::Packet(packet) => {
						let addr = packet.addr();
						if let Some(from) = self.remote_clients.get_by_right(&addr) {
							if let Ok(packet) = bincode::deserialize(packet.payload()) {
								interface.receive(*from, packet);
							} else {
								error!(target: "tick@networking.server", "Invalid packet from {}@{}", from, addr);
							}
						} else if let std::collections::hash_map::Entry::Vacant(e) =
							self.remote_establishing.entry(addr)
						{
							debug!(target: "tick@networking.server", "Received object from new point {}", addr);
							if packet.payload() == [0x69] {
								e.insert(interface.establishing());
							}
						} else {
							match self
								.remote_establishing
								.get_mut(&addr)
								.unwrap()
								.receive(packet.payload())
							{
								Ok(EstablishingStatus::Respond(data)) => {
									// unwrap is safe according to the method send@Socket.
									socket
										.get_packet_sender()
										.send(laminar::Packet::reliable_unordered(addr, data))
										.unwrap();
								}
								Ok(EstablishingStatus::Connect(connection_data)) => {
									let client = rustaria_util::uuid();
									interface.connected(client, connection_data);
									self.remote_clients.insert(client, addr);
									self.remote_establishing.remove(&addr);
								}
								Err(err) => {
									error!(target: "tick@networking.server", "Error connecting to {}, {}", addr, err);
									self.remote_establishing.remove(&addr);
								}
							};
						}
					}
					SocketEvent::Connect(_) => {}
					SocketEvent::Timeout(_) => {}
					SocketEvent::Disconnect(addr) => {
						if let Some(value) = self.remote_clients.get_by_right(&addr) {
							self.disconnected.write().unwrap().insert(*value);
						}
					}
				}
			}

			for client in self.disconnected.write().unwrap().drain() {
				interface.disconnected(client);
				self.remote_clients.remove_by_left(&client);
				if self.local_connections.contains_key(&client) {
					warn!(target: "tick@networking.server", "Tried to disconnect local client.");
				}
			}
		}
	}
}

pub enum ClientNetworking<I: crate::Packet, O: crate::Packet> {
	Local(Sender<O>, Receiver<I>),
	Remote(Box<Socket>, SocketAddr),
}

impl<I: crate::Packet, O: crate::Packet> ClientNetworking<I, O> {
	pub fn join_local<SJ>(
		networking: &mut ServerNetworking<O, I, SJ>,
		join_data: SJ,
	) -> ClientNetworking<I, O> {
		let outbound = unbounded();
		let inbound = unbounded();

		let token = Token::new_v4();
		networking
			.local_connections
			.insert(token, (inbound.0, outbound.1));
		networking
			.new_players
			.write()
			.unwrap()
			.push((token, join_data));

		ClientNetworking::Local(outbound.0, inbound.1)
	}

	pub fn send(&self, packet: O) -> Result<()> {
		trace!(target: "tick@networking.client", "Sending packet: {packet:?}");
		match self {
			ClientNetworking::Local(sender, _) => {
				sender.send(packet).unwrap();
			}
			ClientNetworking::Remote(socket, addr) => {
				socket.get_packet_sender().send(Packet::reliable_unordered(
					*addr,
					bincode::serialize(&packet)?,
				))?;
			}
		}

		Ok(())
	}

	pub fn poll<E, F: FnMut(I) -> core::result::Result<(), E>>(
		&mut self,
		mut consumer: F,
	) -> core::result::Result<(), E> {
		match self {
			ClientNetworking::Local(_, receiver) => {
				while let Ok(packet) = receiver.try_recv() {
					trace!(target: "tick@networking.client", "Received packet: {packet:?}");
					consumer(packet)?;
				}
			}
			ClientNetworking::Remote(_, _) => {
				todo!()
			}
		}

		Ok(())
	}
}
