use std::collections::HashMap;
use crate::chunk::Chunk;

use serde::Serialize;
use serde::Deserialize;
use crate::api::Rustaria;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModListPacket {
    // List of (mod name, mod version)
    pub data: HashMap<String, String>
}

impl ModListPacket {
    pub fn new(rustaria: &Rustaria) -> ModListPacket {
        let mut out = HashMap::new();
        for (name, plugin) in &rustaria.plugins.0 {
            out.insert(name.clone(), plugin.manifest.version.clone());
        }

        ModListPacket {
            data: out
        }

    }
}

