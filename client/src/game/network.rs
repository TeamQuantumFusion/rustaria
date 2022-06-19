use std::net::SocketAddr;

use crossbeam::channel::{unbounded, Receiver, Sender};
use eyre::{Result, WrapErr};
use laminar::{Packet, Socket, SocketEvent};
use rustaria::{
	network::{
		packet::{ClientBoundPacket, ServerBoundPacket},
		util::Connector,
		ServerNetwork,
	},
	KERNEL_VERSION,
};
use semver::Version;
use tracing::error;

pub enum ClientNetworkSystem {
	Integrated {
		sender: Sender<ServerBoundPacket>,
		receiver: Receiver<ClientBoundPacket>,
	},
	Remote {
		socket: Box<Socket>,
		addr: SocketAddr,
	},
}

impl ClientNetworkSystem {
	pub fn new_integrated() -> (ClientNetworkSystem, ServerNetwork) {
		let (c_sender, c_receiver) = unbounded();
		let (s_sender, s_receiver) = unbounded();

		(
			ClientNetworkSystem::Integrated {
				sender: s_sender,
				receiver: c_receiver,
			},
			ServerNetwork {
				sender: c_sender,
				receiver: s_receiver,
			},
		)
	}

	pub fn send(&self, packet: impl Into<ServerBoundPacket>) -> Result<()> {
		let packet = packet.into();
		match self {
			ClientNetworkSystem::Integrated { sender, .. } => {
				sender.send(packet)?;
			}
			ClientNetworkSystem::Remote { socket, addr } => {
				// TODO compression
				let payload =
					bincode::serialize(&packet).wrap_err("Failed to serialize packet.")?;
				socket
					.get_packet_sender()
					.send(Packet::reliable_unordered(*addr, payload))
					.wrap_err("Failed to send packet.")?;
			}
		}
		Ok(())
	}

	pub fn poll(&self) -> Vec<ClientBoundPacket> {
		match self {
			ClientNetworkSystem::Integrated { receiver, .. } => receiver.try_iter().collect(),
			ClientNetworkSystem::Remote { socket, .. } => {
				let receiver = socket.get_event_receiver();
				let mut out = Vec::new();
				while let Ok(SocketEvent::Packet(packet)) = receiver.try_recv() {
					match bincode::deserialize(packet.payload()) {
						Ok(value) => out.push(value),
						Err(error) => {
							error!("Failed to deserialize packet {}", error);
						}
					}
				}
				out
			}
		}
	}

	///
	/// # Packet layout
	/// <- Rustaria Kernel version
	/// -> if 0 { cancel }
	fn new_remote(addr: SocketAddr) -> Result<(), ConnectionError> {
		use ConnectionError::*;
		let mut connector = Connector::new(addr).map_err(BindError)?;

		// <- Rustaria Kernel version
		connector.send(&KERNEL_VERSION).map_err(SendKernelVersion)?;

		// -> server kernel version
		let server_version: Version = connector.receive().map_err(ReceiveKernelVersion)?;
		if server_version != KERNEL_VERSION {
			return Err(WrongKernelVersion(server_version));
		}

		// <- Registry hash

		Ok(())
	}
}

pub enum ConnectionError {
	BindError(laminar::ErrorKind),
	SendKernelVersion(eyre::Report),
	ReceiveKernelVersion(eyre::Report),
	WrongKernelVersion(Version),
}
