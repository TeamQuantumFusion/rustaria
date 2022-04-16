use std::fmt::Debug;

use crossbeam::channel::SendError;
use serde::{de::DeserializeOwned, Serialize};

use rustaria_util::Uuid;
use thiserror::Error;

pub mod networking;
pub mod packet;

pub type Token = Uuid;

pub trait Packet: Serialize + DeserializeOwned + Clone + Debug {}

/// A NetworkInterface takes care of handling events that come from the other clients.
pub trait NetworkInterface<I, O, C>
where
    I: Packet,
    O: Packet,
{
    fn receive(&mut self, from: Token, packet: I);
    fn disconnected(&mut self, client: Token);
    fn connected(&mut self, client: Token, connection_data: C);
    fn establishing(&mut self) -> Box<dyn EstablishingInstance<C>>;
}

/// An EstablishingInstance is the first connection step which takes care of handshaking.
pub trait EstablishingInstance<C> {
    fn receive(&mut self, data: &[u8]) -> Result<EstablishingStatus<C>>;
}

pub enum EstablishingStatus<C> {
    Respond(Vec<u8>),
    Connect(C),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bincode serialization error")]
    Serialization(#[from] bincode::Error),

    #[error("Compression error")]
    Compression(#[from] lz4_flex::block::CompressError),

    #[error("Decompression error")]
    Decompression(#[from] lz4_flex::block::DecompressError),

    #[error("Remote Networking error")]
    Remote(#[from] laminar::ErrorKind),

    #[error("Local channel error")]
    Channel,
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::Channel
    }
}
