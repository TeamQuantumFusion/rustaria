use serde::{de::DeserializeOwned, Serialize};

pub trait Packet: Send + 'static + Serialize + DeserializeOwned {}

impl<P: Send + 'static + Serialize + DeserializeOwned> Packet for P {}

pub trait PacketSetup {
	type Client: Packet;
	type Server: Packet;
}
