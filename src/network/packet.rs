use crate::chunk::Chunk;

use serde::Serialize;
use serde::Deserialize;

// server > client
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerPacket {
    Chunk {
        data: Box<Chunk>
    },
    FuckOff
}

// client > server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientPacket {
    ILoveYou
}
