use std::collections::HashMap;

use crossbeam::channel::{Receiver, Sender};

use crate::packet::Packet;
use crate::server::ServerTickData;
use crate::Token;

pub struct Integrated<I: Packet, O: Packet> {
	pub(crate) join_players: Vec<Token>,
	pub(crate) leave_players: Vec<Token>,
	pub(crate) integrated_connections: HashMap<Token, (Sender<O>, Receiver<I>)>,
}

impl<I: Packet, O: Packet> Integrated<I, O> {
	pub fn new() -> crate::Result<Integrated<I, O>> {
		Ok(Integrated {
			join_players: vec![],
			leave_players: vec![],
			integrated_connections: Default::default(),
		})
	}

	pub fn leave(&mut self, token: Token) {
		if self.integrated_connections.remove(&token).is_some() {
			self.leave_players.push(token);
		}
	}

	pub fn tick(&mut self, data: &mut ServerTickData<I>) -> crate::Result<()> {
		for token in self.join_players.drain(..) {
			data.to_connect.push(token);
		}

		for token in self.leave_players.drain(..) {
			data.to_disconnect.push(token);
		}

		for (from, (_, receiver)) in &self.integrated_connections {
			while let Ok(packet) = receiver.try_recv() {
				data.received.push((*from, packet));
			}
		}
		Ok(())
	}

	pub fn send(&self, to: Token, packet: O) -> crate::Result<()> {
		if let Some((sender, _)) = self.integrated_connections.get(&to) {
			sender.send(packet)?;
		}
		Ok(())
	}

	pub fn send_all(&self, packet: O) -> crate::Result<()> {
		for (sender, _) in self.integrated_connections.values() {
			sender.send(packet.clone())?;
		}

		Ok(())
	}
}
