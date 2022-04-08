use serde::{Deserialize, Serialize};

use rustaria_network::Packet;

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerPacket {}

impl Packet for ServerPacket {}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientPacket {}

impl Packet for ClientPacket {}
