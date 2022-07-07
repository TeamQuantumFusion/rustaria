use rsa_core::{
	err::Result,
	sync::channel::{Receiver, Sender},
};

use crate::{Packet, PacketSetup, Token};

pub struct ServerSender<'src, P: Packet> {
	send: Box<dyn Fn(Token, P) -> Result<()> + 'src>,
}

impl<'src, P: Packet> ServerSender<'src, P> {
	pub fn send(&self, to: Token, packet: P) -> Result<()> { (self.send)(to, packet) }

	pub fn map<T: Packet + Into<P>>(&self) -> ServerSender<T> {
		ServerSender {
			send: Box::new(|to, packet| self.send(to, packet.into())),
		}
	}
}

pub enum ServerNetwork<P: PacketSetup> {
	Local(LocalServerNetwork<P>),
}

impl<P: PacketSetup> ServerNetwork<P> {
	pub fn sender(&self) -> ServerSender<P::Client> {
		ServerSender {
			send: match self {
				ServerNetwork::Local(network) => {
					let sender = network.sender.clone();
					Box::new(move |_, packet| {
						sender.send(packet)?;
						Ok(())
					})
				}
			},
		}
	}

	pub fn send(&self, _to: Token, packet: P::Client) -> Result<()> {
		match self {
			ServerNetwork::Local(network) => {
				network.sender.send(packet)?;
			}
		}
		Ok(())
	}

	pub fn poll(&self) -> Vec<(Token, P::Server)> {
		match self {
			ServerNetwork::Local(network) => network
				.receiver
				.try_iter()
				.map(|packet| (Token(), packet))
				.collect(),
		}
	}
}

pub struct LocalServerNetwork<P: PacketSetup> {
	pub(crate) sender: Sender<P::Client>,
	pub(crate) receiver: Receiver<P::Server>,
}
