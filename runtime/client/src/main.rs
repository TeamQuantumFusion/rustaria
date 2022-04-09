use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use glfw::{Key, MouseButton};

use rustaria::api::prototype::tile::TilePrototype;
use rustaria::api::Api;
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::network::Networking;
use rustaria::world::chunk::{Chunk, ChunkLayer};
use rustaria::world::World;
use rustaria::{Server, UPS};
use rustaria_api::tag::Tag;
use rustaria_controller::button::{ButtonKey, HoldSubscriber, TriggerSubscriber};
use rustaria_controller::ControllerHandler;
use rustaria_graphics::ty::{Player};
use rustaria_graphics::RenderHandler;
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_util::ty::{ChunkPos, CHUNK_SIZE};
use rustaria_util::{Result, warn};
use crate::controller::PrintSubscriber;

mod network;
mod controller;

const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS as u64);

fn main() {
    rustaria_util::initialize().unwrap();

    let mut api = Api::new();
    api.reload().unwrap();

    let mut server = Server {
        network: Networking {
            internal: ServerNetworking::new(None).unwrap(),
        },
        world: World::new(),
    };
    let networking = ClientNetworking::join_local(&mut server.network.internal);

    let air = api
        .get_registry::<TilePrototype>()
        .create_from_tag(&Tag::from_str("rustaria:dirt").unwrap())
        .unwrap();

    server.world.put_chunk(
        ChunkPos { x: 0, y: 0 },
        Chunk {
            tiles: ChunkLayer::new([[air; CHUNK_SIZE]; CHUNK_SIZE]),
        },
    );

    let mut bindings = HashMap::new();
    bindings.insert("up".to_string(), ButtonKey::Keyboard(Key::W));
    bindings.insert("down".to_string(), ButtonKey::Keyboard(Key::S));
    bindings.insert("left".to_string(), ButtonKey::Keyboard(Key::A));
    bindings.insert("right".to_string(), ButtonKey::Keyboard(Key::D));
    bindings.insert("zoom_in".to_string(), ButtonKey::Keyboard(Key::R));
    bindings.insert("zoom_out".to_string(), ButtonKey::Keyboard(Key::F));

    let up = HoldSubscriber::new();
    let down = HoldSubscriber::new();
    let left = HoldSubscriber::new();
    let right = HoldSubscriber::new();
    let zoom_in = TriggerSubscriber::new();
    let zoom_out = TriggerSubscriber::new();


    let mut controller = ControllerHandler::new(bindings);
    controller.subscribe(Box::new(up.clone()), "up".to_string());
    controller.subscribe(Box::new(down.clone()), "down".to_string());
    controller.subscribe(Box::new(left.clone()), "left".to_string());
    controller.subscribe(Box::new(right.clone()), "right".to_string());
    controller.subscribe(Box::new(zoom_in.clone()), "zoom_in".to_string());
    controller.subscribe(Box::new(zoom_out.clone()), "zoom_out".to_string());
    controller.apply();

    Client {
        render: RenderHandler::new(&api).unwrap(),
        up,
        down,
        left,
        right,
        zoom_in,
        api,
        world: Some(ClientWorld {
            networking,
            integrated: Some(Box::new(server)),
        }),
        controller,
        zoom_out
    }
    .run();
}

pub struct Client {
    pub api: Api,
    pub render: RenderHandler,
    pub up: HoldSubscriber,
    pub down: HoldSubscriber,
    pub left: HoldSubscriber,
    pub right: HoldSubscriber,
    pub zoom_in: TriggerSubscriber,
    pub zoom_out: TriggerSubscriber,
    pub controller: ControllerHandler,
    pub world: Option<ClientWorld>,
}

pub struct ClientWorld {
    pub networking: ClientNetworking<ServerPacket, ClientPacket>,
    pub integrated: Option<Box<Server>>,
}

impl Client {
    pub fn run(&mut self) {
        let mut previous_update = Instant::now();
        let mut lag = Duration::ZERO;
        let mut view = Player { pos: [0.0, 0.0], zoom: 30.0 };
        while self.render.alive() {
            let duration = previous_update.elapsed();
            lag += duration;
            previous_update = Instant::now();

            self.render.poll(|event| {
                self.controller.consume(event);
            });

            while lag >= UPDATE_TIME {
                self.tick().unwrap();
                lag -= UPDATE_TIME;
            }

            if self.up.held() { view.pos[1] += 0.5; }
            if self.down.held() { view.pos[1] -= 0.5; }
            if self.left.held() { view.pos[0] -= 0.5; }
            if self.right.held() { view.pos[0] += 0.5; }
            if self.zoom_in.triggered() {
                view.zoom += 5.0;
            }
            if self.zoom_out.triggered() {
                view.zoom -= 5.0;
            }

            self.render.draw(&view);
        }
    }

    fn tick(&mut self) -> Result<()>{
        if let Some(world) = &mut self.world {
            if let Some(integrated) = &mut world.integrated {
                integrated.tick();
            }

            world.networking.poll(|packet| {
                match packet {
                    ServerPacket::Chunks(chunks) => {
                        match chunks.export() {
                            Ok(chunks) => {
                                for (pos, chunk) in chunks.chunks {
                                    self.render.world_renderer.submit_chunk(&self.api, pos, &chunk);
                                }
                            }
                            Err(chunks) => {
                                warn!("Could not deserialize chunk packet. {}", chunks)
                            }
                        }
                    }
                }
            })
        }

        Ok(())
    }
}


