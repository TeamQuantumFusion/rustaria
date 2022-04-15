use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::sync::Arc;
use std::time::{Duration, Instant};

use glfw::{Key, WindowEvent};
use rayon::ThreadPoolBuilder;

use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::network::Networking;
pub use rustaria::prototypes;
pub use rustaria::pt;
use rustaria::world::chunk::Chunk;
use rustaria::world::entity::EntityHandler;
use rustaria::world::World;
use rustaria::{Server, UPS};
use rustaria_api::ty::Prototype;
use rustaria_api::{Api, Carrier};

use rustaria_controller::button::{ButtonKey, HoldSubscriber, TriggerSubscriber};
use rustaria_controller::ControllerHandler;
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_rendering::chunk_drawer::ChunkDrawer;
use rustaria_util::ty::pos::Pos;
use rustaria_util::ty::ChunkPos;
use rustaria_util::ty::CHUNK_SIZE;
use rustaria_util::{info, warn, Result};
use rustariac_backend::ty::Viewport;
use rustariac_backend::ClientBackend;
use rustariac_glium_backend::GliumBackend;

mod controller;

const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS as u64);

fn main() {
    rustaria_util::initialize().unwrap();

    let backend = ClientBackend::new(GliumBackend::new).unwrap();

    let mut carrier = Carrier::new();
    let mut dir = std::env::current_dir().unwrap();
    dir.push("plugins");
    let mut api = Api::new(dir).unwrap();

    // Reload your mom
    let mut reload = api.reload(&mut carrier);
    prototypes!({ reload.add_reload_registry::<P>() });
    reload.reload();
    prototypes!({ reload.add_apply_registry::<P>() });
    reload.apply();

    let mut server = Server {
        carrier: carrier.clone(),
        network: Networking::new(ServerNetworking::new(None).unwrap()),
        world: World::new(carrier.clone(), 12).unwrap(),
    };

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

    let mut sprites = HashSet::new();
    let instance = carrier.lock();

    prototypes!({
        for prototype in instance.get_registry::<P>().iter() {
            prototype.get_sprites(&mut sprites);
        }
    });

    backend.instance_mut().supply_atlas(&api, sprites);

    // server.world.entities.spawn(
    //     instance
    //         .get_registry::<EntityPrototype>()
    //         .get_id(&Tag::new("rustaria:bunne".to_string()).unwrap())
    //         .unwrap(),
    //     Pos { x: 0.0, y: 0.0 },
    // );

    let drawer = ChunkDrawer::new(&carrier, &backend);
    Client {
        up,
        down,
        left,
        right,
        zoom_in,
        carrier: carrier.clone(),
        world: Some(ClientWorld {
            carrier: carrier.clone(),
            networking: ClientNetworking::join_local(&mut server.network.internal),
            chunks: Default::default(),
            entities: EntityHandler::new(
                &carrier,
                Arc::new(ThreadPoolBuilder::new().build().unwrap()),
            ),
            chunk_drawer: drawer,
            old_chunk: ChunkPos { x: 0, y: 0 },
            old_zoom: 0.0,
            integrated: Some(Box::new(server)),
        }),
        controller,
        zoom_out,
        view: Viewport {
            position: [0.0, 0.0],
            zoom: 30.0,
        },
        backend,
    }
    .run();
}

pub struct Client {
    pub carrier: Carrier,
    pub up: HoldSubscriber,
    pub down: HoldSubscriber,
    pub left: HoldSubscriber,
    pub right: HoldSubscriber,
    pub zoom_in: TriggerSubscriber,
    pub zoom_out: TriggerSubscriber,
    pub controller: ControllerHandler,
    pub view: Viewport,

    pub world: Option<ClientWorld>,
    pub backend: ClientBackend,
}

impl Client {
    pub fn run(&mut self) {
        let mut last_tick = Instant::now();
        let mut last_delta = 0f32;

        while !self.backend.instance().backend.window().should_close() {
            for event in self.backend.instance_mut().backend.poll_events() {
                match event {
                    WindowEvent::Scroll(_, y) => {
                        self.view.zoom += y as f32;
                    }
                    _ => {}
                }

                self.controller.consume(event);
            }

            while last_tick.elapsed() >= UPDATE_TIME {
                self.tick().unwrap();
                last_tick.add_assign(UPDATE_TIME);
            }

            let delta = ((last_tick.elapsed().as_secs_f32() / UPDATE_TIME.as_secs_f32())
                - last_delta)
                .abs();
            self.draw(delta);
            last_delta = delta;
        }
    }

    fn tick(&mut self) -> Result<()> {
        if let Some(world) = &mut self.world {
            world.tick(&self.view, &self.backend);
        }

        Ok(())
    }

    fn draw(&mut self, delta: f32) {
        let x = self.view.zoom / 30.0;
        if self.up.held() {
            self.view.position[1] += 1.6 * delta * x;
        }
        if self.down.held() {
            self.view.position[1] -= 1.6 * delta * x;
        }
        if self.left.held() {
            self.view.position[0] -= 1.6 * delta * x;
        }
        if self.right.held() {
            self.view.position[0] += 1.6 * delta * x;
        }
        if self.zoom_in.triggered() {
            self.view.zoom += 5.0;
        }
        if self.zoom_out.triggered() {
            self.view.zoom -= 5.0;
        }

        if let Some(world) = &mut self.world {
            world.draw(&self.view);
        }

        self.backend.instance_mut().backend.draw(&self.view);
    }
}

pub struct ClientWorld {
    pub carrier: Carrier,
    pub networking: ClientNetworking<ServerPacket, ClientPacket>,
    pub chunks: HashMap<ChunkPos, ChunkHolder>,
    pub entities: EntityHandler,
    pub chunk_drawer: ChunkDrawer,
    pub old_chunk: ChunkPos,
    pub old_zoom: f32,
    pub integrated: Option<Box<Server>>,
}

impl ClientWorld {
    pub fn tick(&mut self, view: &Viewport, backend: &ClientBackend) {
        if let Ok(chunk) = ChunkPos::try_from(Pos {
            x: view.position[0],
            y: view.position[1],
        }) {
            if chunk != self.old_chunk || view.zoom != self.old_zoom {
                info!("{:?}", view);
                let width = (view.zoom / CHUNK_SIZE as f32) as i32;
                let height = ((view.zoom * backend.screen_y_ratio()) / CHUNK_SIZE as f32) as i32;
                let mut requested = Vec::new();
                for x in -width..width {
                    for y in -height..height {
                        if let Some(pos) = chunk.offset([x, y]) {
                            if let Entry::Vacant(e) = self.chunks.entry(pos) {
                                e.insert(ChunkHolder::Requested);
                                requested.push(pos);
                            }
                        }
                    }
                }

                self.chunk_drawer.mark_dirty();
                if !requested.is_empty() {
                    self.networking
                        .send(ClientPacket::RequestChunks(requested))
                        .unwrap();
                }
                self.old_chunk = chunk;
                self.old_zoom = view.zoom;
            }
        }

        if let Some(integrated) = &mut self.integrated {
            integrated.tick().unwrap();
        }

        self.networking.poll(|packet| match packet {
            ServerPacket::Chunks(chunks) => match chunks.export() {
                Ok(chunks) => {
                    for (pos, chunk) in chunks.chunks {
                        self.chunk_drawer.submit(pos, &chunk);
                        self.chunks.insert(pos, ChunkHolder::Active(chunk));
                    }
                }
                Err(chunks) => {
                    warn!("Could not deserialize chunk packet. {}", chunks)
                }
            },
            ServerPacket::NewEntity(id, pos) => {
                self.entities.spawn(id, pos);
                info!("{id:?}");
            }
        })
    }

    pub fn draw(&mut self, view: &Viewport) {
        self.chunk_drawer.draw(view);
    }
}

pub enum ChunkHolder {
    Active(Chunk),
    Requested,
}
