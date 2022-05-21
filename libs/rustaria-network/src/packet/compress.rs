use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

impl<P: Serialize + DeserializeOwned> Compress<P> {
	pub fn new(packet: &P) -> crate::Result<Compress<P>> {
		Ok(Compress {
			data: lz4_flex::compress_prepend_size(&bincode::serialize(packet)?),
			packet: Default::default(),
		})
	}

	pub fn export(self) -> crate::Result<P> {
		let decompressed = lz4_flex::decompress_size_prepended(self.data.as_slice())?;
		Ok(bincode::deserialize(decompressed.as_slice())?)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Compress<P: Serialize + DeserializeOwned> {
	data: Vec<u8>,
	packet: PhantomData<P>,
}
