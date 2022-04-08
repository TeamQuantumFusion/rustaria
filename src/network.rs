use rustaria_network::networking::ServerNetworking;

use crate::{ClientPacket, ServerPacket};
use crate::network::join::PlayerJoinData;

pub mod join;
pub mod packet;

pub struct Networking {
    pub internal: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>,
}
