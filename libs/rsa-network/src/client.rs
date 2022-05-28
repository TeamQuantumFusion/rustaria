use std::net::SocketAddr;

use crossbeam::channel::{Receiver, Sender, TryRecvError};
use laminar::{Socket, SocketEvent};

use crate::packet::Packet;
use crate::Token;

// the reason the enum is wrapped is because i think having an enum for client and a struct for server is cringe in highlighting because of my theme.
// So ehm deal with it i guess?
// -- EDIT, actually this also makes the inner fields private so i guess this is another point.
pub struct ClientNetwork<I: Packet, O: Packet> {
	kind: ClientNetworkKind<I, O>,
}

impl<I: Packet, O: Packet> ClientNetwork<I, O> {
	pub fn new_integrated(
		integrated: &mut crate::server::integrated::Integrated<O, I>,
	) -> crate::Result<ClientNetwork<I, O>> {
		let token = Token::new_v4();
		integrated.join_players.push(token);
		let (s_out, c_in) = crossbeam::channel::unbounded();
		let (c_out, s_in) = crossbeam::channel::unbounded();
		integrated
			.integrated_connections
			.insert(token, (s_out, s_in));

		Ok(ClientNetwork {
			kind: ClientNetworkKind::Integrated {
				token,
				send: c_out,
				receive: c_in,
			},
		})
	}

	pub fn new_remote(_addr: SocketAddr) -> crate::Result<ClientNetwork<I, O>> {
		todo!()
	}

	pub fn tick(&mut self) -> crate::Result<ClientTickData<I>> {
		let mut data = Vec::new();
		match &self.kind {
			ClientNetworkKind::Integrated { receive, .. } => loop {
				match receive.try_recv() {
					Ok(value) => data.push(value),
					Err(TryRecvError::Empty) => break,
					Err(TryRecvError::Disconnected) => {
						return Ok(ClientTickData::Disconnected);
					}
				}
			},
			ClientNetworkKind::Remote { addr, socket } => loop {
				match socket.get_event_receiver().try_recv() {
					Ok(SocketEvent::Packet(packet)) => {
						if packet.addr() == *addr {
							data.push(bincode::deserialize(packet.payload())?);
						}
					}
					Err(TryRecvError::Disconnected) | Ok(SocketEvent::Disconnect(_)) => {
						return Ok(ClientTickData::Disconnected);
					}
					Err(TryRecvError::Empty) => break,
					_ => {}
				}
			},
		}

		Ok(ClientTickData::Received(data))
	}

	pub fn send(&self, packet: O) -> crate::Result<()> {
		match &self.kind {
			ClientNetworkKind::Integrated { send, .. } => send.send(packet)?,
			ClientNetworkKind::Remote { addr, socket } => {
				socket.get_packet_sender().send(
					packet
						.get_desc()
						.packet(*addr, bincode::serialize(&packet)?),
				)?;
			}
		}

		Ok(())
	}
}

pub enum ClientTickData<I: Packet> {
	Received(Vec<I>),
	Disconnected,
}

pub enum ClientNetworkKind<I: Packet, O: Packet> {
	Integrated {
		token: Token,
		send: Sender<O>,
		receive: Receiver<I>,
	},
	Remote {
		addr: SocketAddr,
		socket: Box<Socket>,
	},
}
