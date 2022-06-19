use rsa_core::logging::trace;
use crate::packet::Packet;
use crate::server::integrated::Integrated;
use crate::server::remote::Remote;
use crate::Token;

pub mod integrated;
pub mod remote;

pub struct ServerNetwork<I: Packet, O: Packet> {
	pub integrated: Option<Integrated<I, O>>,
	pub remote: Option<Remote<I, O>>,
}

impl<I: Packet, O: Packet> ServerNetwork<I, O> {
	pub fn tick(&mut self) -> crate::Result<ServerTickData<I>> {
		let mut data = ServerTickData {
			received: vec![],
			to_disconnect: vec![],
			to_connect: vec![],
		};
		if let Some(integrated) = &mut self.integrated {
			integrated.tick(&mut data)?;
		}

		if let Some(remote) = &mut self.remote {
			remote.tick(&mut data)?;
		}

		Ok(data)
	}

	pub fn send(&self, to: Token, packet: O) -> crate::Result<()> {
		if let Some(remote) = &self.remote {
			remote.send(to, &packet)?;
		}

		if let Some(integrated) = &self.integrated {
			integrated.send(to, packet)?;
		}

		Ok(())
	}

	pub fn send_all(&self, packet: O) -> crate::Result<()> {
		if let Some(remote) = &self.remote {
			remote.send_all(&packet)?;
		}

		if let Some(integrated) = &self.integrated {
			integrated.send_all(packet)?;
		}

		Ok(())
	}
}

pub struct ServerTickData<I: Packet> {
	pub received: Vec<(Token, I)>,
	pub to_disconnect: Vec<Token>,
	pub to_connect: Vec<Token>,
}
