use crossbeam::channel::SendError;
use thiserror::Error;
use rsa_core::ty::Uuid;

pub mod client;
pub mod packet;
pub mod server;
pub mod tunnel;

pub type Token = Uuid;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Serialization error")]
	Serialization(#[from] bincode::Error),

	#[error("Compression error")]
	Compression(#[from] lz4_flex::block::CompressError),

	#[error("Decompression error")]
	Decompression(#[from] lz4_flex::block::DecompressError),

	#[error("Networking infrastructure error")]
	Remote(#[from] laminar::ErrorKind),

	#[error("Channel communication failure")]
	Channel,

	#[error("The server type does not support this given feature.")]
	UnsupportedServerKind,
}

impl<T> From<SendError<T>> for Error {
	fn from(_: SendError<T>) -> Self {
		Error::Channel
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
