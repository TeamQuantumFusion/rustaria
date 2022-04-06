use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::RwLock;
use std::time::Instant;

use bimap::BiMap;
use laminar::{Socket, SocketEvent};

use rustaria_util::{debug, error};

use crate::{
    EstablishingInstance, EstablishingStatus, NetworkBackend, NetworkInterface, Packet, Token,
};

pub struct RemoteBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    socket: Socket,
    clients: BiMap<SocketAddr, Token>,
    establishing: HashMap<SocketAddr, EI>,
    disconnected: RwLock<HashSet<Token>>,
    connected: HashSet<SocketAddr>,
    params: PhantomData<(I, O, C)>,
}

impl<I, O, EI, C> RemoteBackend<I, O, EI, C>
    where
        I: Packet,
        O: Packet,
        EI: EstablishingInstance<C>,
{
    pub fn new(addr: SocketAddr) -> crate::Result<RemoteBackend<I, O, EI, C>> {
        Ok(RemoteBackend {
            socket: Socket::bind(addr)?,
            clients: Default::default(),
            establishing: Default::default(),
            disconnected: Default::default(),
            connected: Default::default(),
            params: Default::default()
        })
    }

}

impl<I, O, EI, C> RemoteBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn send_internal(&self, addr: &SocketAddr, payload: Vec<u8>) -> crate::Result<()> {
        self.socket
            .get_packet_sender()
            .send(laminar::Packet::reliable_unordered(*addr, payload))?;
        Ok(())
    }
}

impl<I, O, EI, C> NetworkBackend<I, O, EI, C> for RemoteBackend<I, O, EI, C>
where
    I: Packet,
    O: Packet,
    EI: EstablishingInstance<C>,
{
    fn send(&self, to: Token, packet: O) -> crate::Result<()> {
        let payload = bincode::serialize(&packet)?;
        if let Some(addr) = self.clients.get_by_right(&to) {
            self.send_internal(addr, payload);
        } else {
            self.disconnected.write().unwrap().insert(to);
        }

        Ok(())
    }

    fn distribute(&self, from: Token, packet: O) -> crate::Result<()> {
        for to in self.clients.right_values() {
            if *to != from {
                // clone kinda cringe ngl. might replace with direct call
                self.send(*to, packet.clone())?;
            }
        }
        Ok(())
    }

    fn poll<NI: NetworkInterface<I, O, C, EI>>(&mut self, interface: &mut NI) {
        // kinda cringe ngl, maybe we should create a thread separate for this.
        self.socket.manual_poll(Instant::now());

        while let Ok(event) = self.socket.get_event_receiver().try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let addr = packet.addr();
                    if let Some(from) = self.clients.get_by_left(&addr) {
                        if let Ok(packet) = bincode::deserialize(packet.payload()) {
                            interface.receive(*from, packet);
                        } else {
                            error!("Invalid packet from {}@{}", from, addr);
                        }
                    } else if self.establishing.contains_key(&addr) {
                        match self
                            .establishing
                            .get_mut(&addr)
                            .unwrap()
                            .receive(packet.payload())
                        {
                            Ok(EstablishingStatus::Respond(data)) => {
                                // unwrap is safe according to the method send@Socket.
                                self.socket
                                    .get_packet_sender()
                                    .send(laminar::Packet::reliable_unordered(addr.clone(), data)).unwrap();
                            }
                            Ok(EstablishingStatus::Connect(connection_data)) => {
                                let client = rustaria_util::uuid();
                                interface.connected(client, connection_data);
                                self.clients.insert(addr, client);
                                self.connected.insert(addr);
                                self.establishing.remove(&addr);
                            }
                            Err(err) => {
                                error!("Error connecting to {}, {}", addr, err);
                                self.establishing.remove(&addr);
                            }
                        };
                    } else {
                        debug!("Received data from new point {}", addr);
                        if packet.payload() == &[0x69] {
                            let mut ei = interface.establishing();
                            self.establishing.insert(addr, ei);
                        }
                    }
                }
                SocketEvent::Connect(_) => {}
                SocketEvent::Timeout(_) => {}
                SocketEvent::Disconnect(addr) => {
                    if let Some(value) = self.clients.get_by_left(&addr) {
                        self.disconnected.write().unwrap().insert(*value);
                    }
                }
            }
        }

        for client in self.disconnected.write().unwrap().drain() {
            interface.disconnected(client);
            self.clients.remove_by_right(&client);
        }
    }
}
