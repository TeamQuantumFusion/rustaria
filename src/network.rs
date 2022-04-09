use rustaria_network::networking::ServerNetworking;

use crate::network::join::PlayerJoinData;
use crate::{ClientPacket, ServerPacket};

pub mod join;
pub mod packet;

pub struct Networking {
    pub internal: ServerNetworking<ClientPacket, ServerPacket, PlayerJoinData>,
}
