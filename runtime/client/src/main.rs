use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::time::{Duration, Instant};

use glfw::{Key, WindowEvent};

use rustaria::{Server, UPS};
use rustaria::api::Api;
use rustaria::api::prototype::tile::TilePrototype;
use rustaria::network::Networking;
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::world::chunk::Chunk;
use rustaria::world::World;
use rustaria_api::lua_runtime::Lua;
use rustaria_controller::button::{ButtonKey, HoldSubscriber, TriggerSubscriber};
use rustaria_controller::ControllerHandler;
use rustaria_graphics::BattleCruiser;
use rustaria_graphics::renderer::RenderingHandler;
use rustaria_graphics::ty::Viewport;
use rustaria_graphics::world_drawer::WorldDrawer;
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_util::{Result, warn};
use rustaria_util::ty::CHUNK_SIZE;
use rustaria_util::ty::ChunkPos;
use rustaria_util::ty::Pos;

mod controller;
mod network;

const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS as u64);

fn main() {
    rustaria_util::initialize().unwrap();

    let lua = Lua::new();
    let mut api = Api::new(&lua);
    api.reload(&lua).unwrap();

    let mut server = Server {
        api: api.clone(),
        network: Networking::new(ServerNetworking::new(None).unwrap()),
        world: World::new(api.clone()).unwrap(),
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
    let instance = api.instance();
    for tag in instance
        .get_registry::<TilePrototype>()
        .entries()
        .iter()
        .filter_map(|prototype| prototype.sprite.as_ref())
    {
        sprites.insert(tag.clone());
    }

    let battle_cruiser = BattleCruiser::operational().unwrap();
    let mut renderer = RenderingHandler::new(&api, sprites);

    let drawer = WorldDrawer::new(&api, &mut renderer);
    Client {
        battle_cruiser,
        renderer,
        up,
        down,
        left,
        right,
        zoom_in,
        api: api.clone(),
        world: Some(ClientWorld {
            api: api.clone(),
            networking: ClientNetworking::join_local(&mut server.network.internal),
            chunks: Default::default(),
            drawer,
            old_chunk: ChunkPos { x: 0, y: 0 },
            old_zoom: 0.0,
            integrated: Some(Box::new(server)),
        }),
        controller,
        zoom_out,
        view: Viewport {
            pos: Pos::from([0.0, 0.0]),
            zoom: 30.0,
        },
    }
    .run();
}

pub struct Client {
    pub api: Api,
    pub battle_cruiser: BattleCruiser,
    pub renderer: RenderingHandler,
    pub up: HoldSubscriber,
    pub down: HoldSubscriber,
    pub left: HoldSubscriber,
    pub right: HoldSubscriber,
    pub zoom_in: TriggerSubscriber,
    pub zoom_out: TriggerSubscriber,
    pub controller: ControllerHandler,

    pub view: Viewport,
    pub world: Option<ClientWorld>,
}

impl Client {
    pub fn run(&mut self) {
        let mut last_tick = Instant::now();
        let mut last_delta = 0f32;
        while self.battle_cruiser.alive() {
            self.battle_cruiser.poll(|event| {
                match event {
                    WindowEvent::Size(width, height) => {
                        self.renderer.resize(width as u32, height as u32);
                    }
                    WindowEvent::Scroll(x, y) => {
                        self.view.zoom += y as f32;
                    }
                    _ => {}
                }

                self.controller.consume(event);
            });

            while last_tick.elapsed() >= UPDATE_TIME {
                self.tick().unwrap();
                last_tick.add_assign(UPDATE_TIME);
            }

            let delta = ((last_tick.elapsed().as_secs_f32() / UPDATE_TIME.as_secs_f32())
                - last_delta)
                .abs();
            self.draw(1.0);
            last_delta = delta;
        }
    }

    fn tick(&mut self) -> Result<()> {
        if let Some(world) = &mut self.world {
            world.tick(&self.view, &mut self.renderer);
        }

        Ok(())
    }

    fn draw(&mut self, delta: f32) {
        if self.up.held() {
            self.view.pos.y += 1.6 * delta;
        }
        if self.down.held() {
            self.view.pos.y -= 1.6 * delta;
        }
        if self.left.held() {
            self.view.pos.x -= 1.6 * delta;
        }
        if self.right.held() {
            self.view.pos.x += 1.6 * delta;
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
        self.battle_cruiser.draw(&mut self.renderer, &self.view);
    }
}

pub struct ClientWorld {
    pub api: Api,
    pub networking: ClientNetworking<ServerPacket, ClientPacket>,
    pub chunks: HashMap<ChunkPos, ChunkHolder>,
    pub drawer: WorldDrawer,
    pub old_chunk: ChunkPos,
    pub old_zoom: f32,
    pub integrated: Option<Box<Server>>,
}

impl ClientWorld {
    pub fn tick(&mut self, view: &Viewport, renderer: &mut RenderingHandler) {
        if let Ok(chunk) = ChunkPos::try_from(view.pos) {
            if chunk != self.old_chunk || view.zoom != self.old_zoom {
                let width = (view.zoom / CHUNK_SIZE as f32) as i32;
                let height = ((view.zoom * renderer.instance().read().unwrap().screen_y_ratio)
                    / CHUNK_SIZE as f32) as i32;
                let mut requested = Vec::new();
                for x in -width..width {
                    for y in -height..height {
                        if let Some(pos) = chunk.offset([x, y]) {
                            if !self.chunks.contains_key(&pos) {
                                self.chunks.insert(pos, ChunkHolder::Requested);
                                requested.push(pos);
                            }
                        }
                    }
                }

                renderer.mark_dirty();
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
            integrated.tick();
        }

        self.networking.poll(|packet| match packet {
            ServerPacket::Chunks(chunks) => match chunks.export() {
                Ok(chunks) => {
                    for (pos, chunk) in chunks.chunks {
                        self.drawer.submit(pos, &chunk);
                        self.chunks.insert(pos, ChunkHolder::Active(chunk));
                    }
                }
                Err(chunks) => {
                    warn!("Could not deserialize chunk packet. {}", chunks)
                }
            },
        })
    }

    pub fn draw(&mut self, view: &Viewport) {
        self.drawer.draw(view);
    }
}

pub enum ChunkHolder {
    Active(Chunk),
    Requested,
}
