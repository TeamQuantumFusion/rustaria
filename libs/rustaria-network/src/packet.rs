use lz4::{Decoder, EncoderBuilder};
use rustaria_util::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompressedPacket<P: Serialize + DeserializeOwned> {
    data: Vec<u8>,
    packet: PhantomData<P>,
}

impl<P: Serialize + DeserializeOwned> CompressedPacket<P> {
    pub fn new(packet: &P) -> Result<CompressedPacket<P>> {
        let data = Vec::new();

        let mut encoder = EncoderBuilder::new().level(4).build(data)?;
        bincode::serialize_into(&mut encoder, packet)?;
        let (data, result) = encoder.finish();
        result?;

        Ok(CompressedPacket {
            data,
            packet: Default::default(),
        })
    }

    pub fn export(self) -> Result<P> {
        let mut data = Vec::new();

        let mut decoder = Decoder::new(self.data.as_slice())?;
        decoder.read_to_end(&mut data)?;
        decoder.finish().1?;

        Ok(bincode::deserialize(data.as_slice())?)
    }
}
