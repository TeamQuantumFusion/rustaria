use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompressedPacket<P: Serialize + DeserializeOwned> {
    data: Vec<u8>,
    packet: PhantomData<P>,
}

impl<P: Serialize + DeserializeOwned> CompressedPacket<P> {
    pub fn new(packet: &P) -> Result<CompressedPacket<P>> {
        Ok(CompressedPacket {
            data: lz4_flex::compress_prepend_size(&bincode::serialize(packet)?),
            packet: Default::default(),
        })
    }

    pub fn export(self) -> Result<P> {
        let decompressed =
            lz4_flex::decompress_size_prepended(self.data.as_slice())?;
        Ok(bincode::deserialize(decompressed.as_slice())?)
    }
}
