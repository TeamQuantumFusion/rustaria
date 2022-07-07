use rsa_network::packet::PacketSetup;
use rsa_player::packet::{ClientBoundPlayerPacket, ServerBoundPlayerPacket};
use rsa_world::{ClientBoundWorldPacket, ServerBoundWorldPacket};

use crate::Rustaria;

impl PacketSetup for Rustaria {
	type Client = ClientBoundPacket;
	type Server = ServerBoundPacket;
}

#[macro_export]
macro_rules! impl_packet {
	($NAME:ident($SERVER:ident, $CLIENT:ident)) => {
		impl From<$SERVER> for ServerBoundPacket {
			fn from(value: $SERVER) -> Self { ServerBoundPacket::$NAME(value) }
		}

		impl From<$CLIENT> for ClientBoundPacket {
			fn from(value: $CLIENT) -> Self { ClientBoundPacket::$NAME(value) }
		}
	};
}


#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerBoundPacket {
	World(ServerBoundWorldPacket),
	Player(ServerBoundPlayerPacket),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundPacket {
	World(ClientBoundWorldPacket),
	Player(ClientBoundPlayerPacket),
}

impl_packet!(World(ServerBoundWorldPacket, ClientBoundWorldPacket));
impl_packet!(Player(ServerBoundPlayerPacket, ClientBoundPlayerPacket));
