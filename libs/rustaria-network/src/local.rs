use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

use std::sync::RwLock;

use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::{EstablishingInstance, NetworkBackend, NetworkInterface, Packet, Token};

pub struct LocalBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
{
    this: Token,
    clients: HashMap<Token, (Sender<O>, Receiver<I>)>,
    connected: RwLock<HashSet<(Token, C)>>,
    disconnected: RwLock<HashSet<Token>>,
    ei: PhantomData<EI>
}

impl<I, O, EI, C> LocalBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    C: Eq + Hash,
    EI: EstablishingInstance<C>,
{
    pub fn new() -> LocalBackend<I, O, EI, C>{
        Self {
            this: rustaria_util::uuid(),
            clients: HashMap::new(),
            connected: RwLock::new(HashSet::new()),
            disconnected: RwLock::new(HashSet::new()),
            ei: Default::default()
        }
    }

    pub fn connect<OC: Eq + Hash>(
        &mut self,
        other: &mut LocalBackend<O, I, EI, OC>,
        this_info: C,
        other_info: OC,
    ) {
        let outbound = unbounded();
        let inbound = unbounded();

        // Link them
        other.clients.insert(self.this, (inbound.0, outbound.1));
        self.clients.insert(other.this, (outbound.0, inbound.1));

        // Connection packets
        other
            .connected
            .write()
            .unwrap()
            .insert((self.this, other_info));
        self.connected
            .write()
            .unwrap()
            .insert((other.this, this_info));
    }
}

impl<I, O, EI, C> NetworkBackend<I, O, EI, C> for LocalBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn send(&self, to: Token, packet: O) -> crate::Result<()> {
        if let Some((sender, _)) = self.clients.get(&to) {
            if sender.send(packet).is_ok() {
                return Ok(());
            }
        }
        // Trigger disconnect to clear all usages of the token.
        // Happens if the client does not exist or if the sender is closed.
        self.disconnected.write().unwrap().insert(to);
        Ok(())
    }

    fn distribute(&self, from: Token, packet: O) -> crate::Result<()> {
        for to in self.clients.keys() {
            if *to != from {
                self.send(*to, packet.clone())?;
            }
        }
        Ok(())
    }

    fn poll(&mut self, interface: &mut impl NetworkInterface<I, O, C, EI>) {
        for (from, (_, receiver)) in &self.clients {
            while let Ok(value) = receiver.try_recv() {
                interface.receive(*from, value);
            }
        }

        for client in self.disconnected.write().unwrap().drain() {
            interface.disconnected(client);
        }

        for (client, connection_data) in self.connected.write().unwrap().drain() {
            interface.connected(client, connection_data);
        }
    }
}
