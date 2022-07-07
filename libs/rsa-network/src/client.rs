use rsa_core::{
	err::Result,
	sync::channel::{Receiver, Sender},
};

use crate::{Packet, PacketSetup};

pub struct ClientSender<'src, P: Packet> {
	send: Box<dyn Fn(P) -> Result<()> + 'src>,
}

impl<'src, P: Packet> ClientSender<'src, P> {
	pub fn send(&self, packet: P) -> Result<()> { (self.send)(packet) }

	pub fn map<T: Packet + Into<P>>(&self) -> ClientSender<T> {
		ClientSender {
			send: Box::new(|packet| self.send(packet.into())),
		}
	}
}

pub struct ClientNetwork<P: PacketSetup> {
	pub(crate) sender: Sender<P::Server>,
	pub(crate) receiver: Receiver<P::Client>,
}

impl<P: PacketSetup> ClientNetwork<P> {
	pub fn sender(&self) -> ClientSender<P::Server> {
		let sender = self.sender.clone();
		ClientSender {
			send: Box::new(move |packet| {
				sender.send(packet)?;
				Ok(())
			}),
		}
	}

	pub fn send(&self, packet: P::Server) -> Result<()> {
		self.sender.send(packet)?;
		Ok(())
	}

	pub fn poll(&self) -> Vec<P::Client> { self.receiver.try_iter().collect() }
}
