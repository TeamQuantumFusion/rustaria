use crate::chunk::Chunk;
use lz4::{Decoder, EncoderBuilder};
use std::collections::HashMap;
use std::io::Read;

use crate::api::Rustaria;
use crate::types::ChunkPos;
use serde::Deserialize;
use serde::Serialize;

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
    pub data: HashMap<String, String>,
}

impl ModListPacket {
    pub fn new(rustaria: &Rustaria) -> ModListPacket {
        let mut out = HashMap::new();
        for (name, plugin) in &rustaria.plugins.0 {
            out.insert(name.clone(), plugin.manifest.version.clone());
        }

        ModListPacket { data: out }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkPacket {
    data: Vec<u8>,
}

impl ChunkPacket {
    pub fn new(chunk: &Chunk) -> eyre::Result<ChunkPacket> {
        let out = Vec::new();
        let mut encoder = EncoderBuilder::new().level(4).build(out)?;
        bincode::serialize_into(&mut encoder, chunk)?;
        let (data, result) = encoder.finish();
        result?;

        Ok(ChunkPacket { data })
    }

    pub fn export(self) -> eyre::Result<Chunk> {
        let mut result = Decoder::new(self.data.as_slice())?;
        let mut out = Vec::new();
        result.read_to_end(&mut out)?;
        result.finish().1?;
        Ok(bincode::deserialize(out.as_slice())?)
    }
}
