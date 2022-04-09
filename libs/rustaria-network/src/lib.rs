use rustaria_util::{Result, Uuid};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

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
