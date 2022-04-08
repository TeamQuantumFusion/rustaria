use serde::{de::DeserializeOwned, Serialize};
use rustaria_util::{Result, Uuid};

pub mod local;
pub mod remote;

pub type Token = Uuid;

pub trait Packet: Serialize + DeserializeOwned + Clone {}

/// The backend handles existing connections.
pub trait NetworkBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn send(&self, to: Token, packet: O) -> Result<()>;
    fn distribute(&self, from: Token, packet: O) -> Result<()>;
    fn poll(&mut self, interface: &mut impl NetworkInterface<I, O, C, EI>);
}

/// A NetworkInterface takes care of handling events that come from the other clients.
pub trait NetworkInterface<I, O, C, EI>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn receive(&mut self, from: Token, packet: I);
    fn disconnected(&mut self, client: Token);
    fn connected(&mut self, client: Token, connection_data: C);
    fn establishing(&mut self) -> EI;
}

/// An EstablishingInstance is the first connection step which takes care of handshaking.
pub trait EstablishingInstance<C> {
    fn receive(&mut self, data: &[u8]) -> Result<EstablishingStatus<C>>;
}

pub enum EstablishingStatus<C> {
    Respond(Vec<u8>),
    Connect(C),
}
