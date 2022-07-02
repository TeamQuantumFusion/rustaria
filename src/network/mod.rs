use crossbeam::channel::{unbounded, Receiver, Sender};
use anyways::Result;

use crate::network::packet::{ClientBoundPacket, ServerBoundPacket};

pub mod packet;
pub mod util;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Token();

pub struct ServerNetwork {
	pub sender: Sender<ClientBoundPacket>,
	pub receiver: Receiver<ServerBoundPacket>,
}

impl ServerNetwork {
	pub fn send(&self, _to: Token, packet: impl Into<ClientBoundPacket>) -> Result<()> {
		self.sender.send(packet.into())?;
		Ok(())
	}

	pub fn poll(&self) -> Vec<(Token, ServerBoundPacket)> {
		self.receiver
			.try_iter()
			.map(|packet| (Token(), packet))
			.collect()
	}
}

pub struct ClientNetwork {
	sender: Sender<ServerBoundPacket>,
	receiver: Receiver<ClientBoundPacket>,
}

impl ClientNetwork {
	pub fn send(&self, packet: impl Into<ServerBoundPacket>) -> Result<()> {
		self.sender.send(packet.into())?;
		Ok(())
	}

	pub fn poll(&self) -> Vec<ClientBoundPacket> { self.receiver.try_iter().collect() }
}

pub fn new_networking() -> (ClientNetwork, ServerNetwork) {
	let (c_sender, c_receiver) = unbounded();
	let (s_sender, s_receiver) = unbounded();

	(
		ClientNetwork {
			sender: s_sender,
			receiver: c_receiver,
		},
		ServerNetwork {
			sender: c_sender,
			receiver: s_receiver,
		},
	)
}
