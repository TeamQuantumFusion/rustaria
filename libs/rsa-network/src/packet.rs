use std::fmt::Debug;
use std::net::SocketAddr;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod compress;

pub trait Packet: Serialize + DeserializeOwned + Clone + Debug + 'static {
	fn get_desc(&self) -> PacketDesc;
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub enum ReliableKind {
	/// This will ensure that a packet will always arrive. If it gets dropped it will get recent on next network update.
	Reliable,
	/// These packets may be dropped at any point in time and have no guarantee that they will arrive.
	Unreliable,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub enum OrderKind {
	/// No arranging will be done.
	None,
	/// Packets will be arranged in sequence.
	/// If an older packet is received after a newer packet it will get dropped.
	Sequenced(Option<u8>),
	/// Packets will be arranged in order.
	Ordered(Option<u8>),
}

pub struct PacketDesc {
	pub reliable: ReliableKind,
	pub ordered: OrderKind,
}

impl PacketDesc {
	pub(crate) fn packet(&self, addr: SocketAddr, payload: Vec<u8>) -> laminar::Packet {
		match self.reliable {
			ReliableKind::Reliable => match self.ordered {
				OrderKind::None => laminar::Packet::reliable_unordered(addr, payload),
				OrderKind::Sequenced(stream_id) => {
					laminar::Packet::reliable_sequenced(addr, payload, stream_id)
				}
				OrderKind::Ordered(stream_id) => {
					laminar::Packet::reliable_ordered(addr, payload, stream_id)
				}
			},
			ReliableKind::Unreliable => match self.ordered {
				OrderKind::None => laminar::Packet::unreliable(addr, payload),
				OrderKind::Sequenced(stream_id) => {
					laminar::Packet::unreliable_sequenced(addr, payload, stream_id)
				}
				OrderKind::Ordered(_) => panic!("Cannot have an ordered ureliable packet kind."),
			},
		}
	}
}
