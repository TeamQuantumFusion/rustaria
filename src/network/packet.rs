use crate::{
	player::{ClientBoundPlayerPacket, ServerBoundPlayerPacket},
	world::{ClientBoundWorldPacket, ServerBoundWorldPacket},
};

#[macro_export]
macro_rules! packet {
	($NAME:ident($SERVER:ident, $CLIENT:ident)) => {
		// Server
		impl From<$SERVER> for $crate::network::packet::ServerBoundPacket {
			fn from(value: $SERVER) -> Self {
				$crate::network::packet::ServerBoundPacket::$NAME(value)
			}
		}
		// Client
		impl From<$CLIENT> for $crate::network::packet::ClientBoundPacket {
			fn from(value: $CLIENT) -> Self {
				$crate::network::packet::ClientBoundPacket::$NAME(value)
			}
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
