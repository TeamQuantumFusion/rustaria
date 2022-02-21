use std::io::Read;

use lz4::{Decoder, EncoderBuilder};
use serde::Deserialize;
use serde::Serialize;

use crate::api::ModList;
use crate::api::Rustaria;
use crate::chunk::Chunk;
use crate::types::ChunkPos;

// server > client
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerPacket {
    Chunk { data: Box<ChunkPacket> },
    FuckOff,
}

impl ServerPacket {
    pub fn get_type_str(&self) -> &str {
        match self {
            ServerPacket::Chunk { .. } => "Chunk",
            ServerPacket::FuckOff => "Funny",
        }
    }
}

// client > server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientPacket {
    ILoveYou,
    RequestChunk(ChunkPos),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModListPacket {
    // List of (mod name, mod version)
    pub mod_list: ModList,
}

impl ModListPacket {
    pub fn new(rustaria: &Rustaria) -> ModListPacket {
        ModListPacket { mod_list: rustaria.mod_list.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkPacket {
    pos: ChunkPos,
    data: Vec<u8>,
}

impl ChunkPacket {
    pub fn new(pos: ChunkPos, chunk: &Chunk) -> eyre::Result<ChunkPacket> {
        let out = Vec::new();
        let mut encoder = EncoderBuilder::new().level(4).build(out)?;
        bincode::serialize_into(&mut encoder, chunk)?;
        let (data, result) = encoder.finish();
        result?;

        Ok(ChunkPacket { pos, data })
    }

    pub fn export(self) -> eyre::Result<(ChunkPos, Chunk)> {
        let mut result = Decoder::new(self.data.as_slice())?;
        let mut out = Vec::new();
        result.read_to_end(&mut out)?;
        result.finish().1?;
        Ok((self.pos, bincode::deserialize(out.as_slice())?))
    }
}
