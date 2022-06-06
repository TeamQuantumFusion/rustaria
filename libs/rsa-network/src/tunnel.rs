use std::marker::PhantomData;
use crate::client::ClientNetwork;
use crate::packet::Packet;
use crate::server::ServerNetwork;

pub trait Tunnel<O: Packet> {
	fn send(&self, packet: O) -> crate::Result<()>;
}

impl<I: Packet, O: Packet> Tunnel<O> for ClientNetwork<I, O> {
	fn send(&self, packet: O) -> crate::Result<()> {
		self.send(packet)
	}
}

impl<I: Packet, O: Packet> Tunnel<O> for ServerNetwork<I, O> {
	fn send(&self, packet: O) -> crate::Result<()> {
		self.send_all(packet)
	}
}

impl<'a, O: Packet, C: Packet + Into<O>> Tunnel<C> for MappedTunnel<'a, O, C> {
	fn send(&self, packet: C) -> crate::Result<()> {
		self.inner.send(packet.into())
	}
}

pub trait MapTunnel<O: Packet, C: Packet + Into<O>> {
	fn map(&self) -> MappedTunnel<O, C>;
}

#[derive(Default)]
pub struct PhantomTunnel<O: Packet> {
	_out: PhantomData<O>
}

pub struct MappedTunnel<'a, O: Packet, C: Packet + Into<O>> {
	inner: &'a dyn Tunnel<O>,
	_current: PhantomData<C>
}

impl<O: Packet, T: Tunnel<O>, C: Packet + Into<O>> MapTunnel<O, C> for T {
	fn map(&self) -> MappedTunnel<O, C> {
		MappedTunnel {
			inner: self,
			_current: Default::default()
		}
	}
}
