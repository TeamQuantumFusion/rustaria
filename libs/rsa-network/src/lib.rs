use rsa_core::{
	err::Result,
	sync::channel::{unbounded, Receiver, Sender},
};

use crate::{
	client::ClientNetwork,
	packet::{Packet, PacketSetup},
	server::{LocalServerNetwork, ServerNetwork},
};

pub mod client;
pub mod packet;
pub mod server;
pub mod util;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Token();

pub fn new_networking<P: PacketSetup>() -> (ClientNetwork<P>, ServerNetwork<P>) {
	let (c_sender, c_receiver) = unbounded();
	let (s_sender, s_receiver) = unbounded();

	(
		ClientNetwork {
			sender: s_sender,
			receiver: c_receiver,
		},
		ServerNetwork::Local(LocalServerNetwork {
			sender: c_sender,
			receiver: s_receiver,
		}),
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn compile() {}
}
