use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use eyre::Result;
use glfw::{Action, Context, Glfw, Key, Modifiers, SwapInterval, Window, WindowEvent};
use structopt::StructOpt;
use tracing::{debug, error, info};

use rustaria::api::prototypes::Prototype;
use rustaria::api::Rustaria;
use rustaria::chunk::Chunk;
use rustaria::network::{PacketDescriptor, PacketOrder, PacketPriority};
use rustaria::network::packet::{ChunkPacket, ClientPacket, ServerPacket};
use rustaria::player::Player;
use rustaria::types::{CHUNK_SIZE, ChunkPos, TilePos};
use rustaria::UPS;
use rustaria::world::World;

use crate::network::{IntegratedServer, ServerCom};
use crate::render::RenderHandler;

mod network;
mod render;

const DEBUG_MOD: Modifiers =
    Modifiers::from_bits_truncate(glfw::ffi::MOD_ALT | glfw::ffi::MOD_SHIFT);
const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS as u64);

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-client", about = "The interactive client of Rustaria")]
struct Opt {
    #[structopt(flatten)]
    inner: rustaria::opt::Opt,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    debug!(?opt, "Got command-line args");
    rustaria::init(opt.inner.verbosity)?;

    let mut client = RustariaClient::new()?;
    let mut previous_update = Instant::now();
    let mut lag = Duration::ZERO;
    while client.running() {
        let duration = previous_update.elapsed();
        lag += duration;
        previous_update = Instant::now();

        while lag >= UPDATE_TIME {
            client.tick();
            lag -= UPDATE_TIME;
        }
        client.draw(duration.as_micros() as f32 / UPDATE_TIME.as_micros() as f32)?;
    }
    Ok(())
}

pub struct RustariaClient {
    render: RenderHandler,
    perf: PerfDisplayerHandler,

    reload_chunks: bool,
    chunks: HashMap<ChunkPos, Chunk>,
    server: Option<Box<dyn ServerCom>>,

    player: Player,
    old_chunk: ChunkPos,

    zoom: f32,
    // this is bad
    w: bool,
    a: bool,
    s: bool,
    d: bool,

    rsa: Rustaria,
    glfw: Glfw,
    glfw_window: Window,
    glfw_events: Receiver<(f64, WindowEvent)>,
}

impl RustariaClient {
    pub fn new() -> eyre::Result<RustariaClient> {
        let title = format!("Rustaria Client {}", env!("CARGO_PKG_VERSION"));
        info!("{}", title);

        info!(target: "api", "Launching Rustaria API");
        let plugins_dir = Path::new("./plugins");
        let mut rsa = Rustaria::new(plugins_dir)?;
        rsa.reload()?;

        info!(target: "render", "Launching GLFW");
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        info!(target: "render", "Creating Window");
        let (mut glfw_window, glfw_events) = glfw
            .create_window(900, 600, &title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        info!(target: "render", "Loading OpenGL backend");
        glfw_window.set_key_polling(true);
        glfw_window.set_size_polling(true);
        glfw_window.set_scroll_polling(true);
        glfw_window.make_current();
        glfw.set_swap_interval(SwapInterval::Sync(1));

        let render = RenderHandler::new(&rsa, &glfw, &glfw_window)?;


        // World creation
        let air_tile = rsa
            .tiles
            .get_id_from_tag(&"rustaria:air".parse()?)
            .expect("Could not find air tile");
        let air_wall = rsa
            .walls
            .get_id_from_tag(&"rustaria:air".parse()?)
            .expect("Could not find air wall");

        let mut chunk = Chunk::new(&rsa, air_tile, air_wall).expect("Could not create empty chunk");

        let dirt_tag = &"rustaria:dirt".parse()?;
        let stone_tag = &"rustaria:stone".parse()?;
        let dirt_prot = rsa.tiles.get_from_tag(dirt_tag).unwrap();
        let dirt_id = rsa.tiles.get_id_from_tag(dirt_tag).unwrap();

        let stone_prot = rsa.tiles.get_from_tag(stone_tag).unwrap();
        let stone_id = rsa.tiles.get_id_from_tag(stone_tag).unwrap();
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                chunk.tiles.grid[y][x] = dirt_prot.create(dirt_id);
            }
        }

        let world = World::new(
            (32, 32),
            vec![chunk; 32 * 32],
        )?;

        let integrated_server = IntegratedServer::new(world, None);
        Ok(RustariaClient {
            glfw,
            glfw_window,
            glfw_events,
            player: Player {
                pos: (0.0, 0.0),
                vel: (0.0, 0.0),
            },
            old_chunk: ChunkPos {
                x: 0,
                y: 0,
            },
            zoom: 24.0,
            w: false,
            a: false,
            s: false,
            d: false,
            render,
            perf: PerfDisplayerHandler {
                old_print: Instant::now(),
                frame_time: Default::default(),
                frame_count: 0,
                update_time: Default::default(),
                update_count: 0,
            },
            reload_chunks: true,
            chunks: Default::default(),
            server: Some(Box::new(integrated_server)),
            rsa,
        })
    }

    pub fn running(&self) -> bool {
        !self.glfw_window.should_close()
    }

    pub fn tick(&mut self) {
        let update_time = Instant::now();
        if let Some(server) = &mut self.server {
            // todo dont unwrap
            if let Some(pos) = TilePos::new(self.player.pos.0 as u64, self.player.pos.1 as u64) {
                if self.reload_chunks || self.old_chunk != pos.chunk_pos() {
                    for y in -8..8 {
                        for x in -8..8 {
                            if let Some(pos) = pos.chunk_pos().offset((x, y)) {
                                if !self.chunks.contains_key(&pos) {
                                    server.send(ClientPacket::RequestChunk(pos), PacketDescriptor { priority: PacketPriority::Reliable, order: PacketOrder::Unordered }).unwrap();
                                }
                            }
                        }
                    }
                    self.old_chunk = pos.chunk_pos();
                    self.reload_chunks = false;

                }
            }

            server.tick(&self.rsa).unwrap();

            for x in server.receive() {
                match x {
                    ServerPacket::Chunk { data } => {
                        match data.export() {
                            Ok((pos, chunk)) => {
                                self.render.world_renderer.submit_chunk(pos, &chunk);
                                self.chunks.insert(pos, chunk);
                            }
                            Err(_) => {
                                error!("oops");
                            }
                        }
                    }
                    ServerPacket::FuckOff => {}
                }
            }
        }

        self.perf.update_time.add_assign(update_time.elapsed());
        self.perf.update_count += 1;
    }

    pub fn draw(&mut self, delta: f32) -> eyre::Result<()> {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.glfw_events) {
            match event {
                WindowEvent::Size(width, height) => {
                    self.render.resize(width as u32, height as u32);
                }
                WindowEvent::Scroll(_, y) => {
                    self.zoom -= y as f32;
                    self.render.world_renderer.qi_u_zoom.set_value(self.zoom);
                }
                WindowEvent::Key(Key::Q, _, Action::Press, DEBUG_MOD) => {
                    self.glfw_window.set_should_close(true)
                }
                WindowEvent::Key(Key::W, _, Action::Press, DEBUG_MOD) => {
                    self.render.wireframe = !self.render.wireframe
                }
                WindowEvent::Key(Key::R, _, Action::Press, DEBUG_MOD) => {
                    self.reload_chunks = true;
                }
                WindowEvent::Key(Key::C, _, Action::Press, DEBUG_MOD) => {
                    self.chunks.clear();
                }
                WindowEvent::Key(Key::W, _, action, _) => self.w = action != Action::Release,
                WindowEvent::Key(Key::A, _, action, _) => self.a = action != Action::Release,
                WindowEvent::Key(Key::S, _, action, _) => self.s = action != Action::Release,
                WindowEvent::Key(Key::D, _, action, _) => self.d = action != Action::Release,
                _ => {}
            }
        }

        self.player.vel.0 = (self.d as u8 as f32 - self.a as u8 as f32) * 32.0;
        self.player.vel.1 = (self.w as u8 as f32 - self.s as u8 as f32) * 32.0;

        self.player.tick(delta);
        self.render.prepare_draw();
        let draw_time = Instant::now();
        self.render.draw(self.player.pos)?;
        self.perf.frame_time.add_assign(draw_time.elapsed());
        self.perf.frame_count += 1;
        self.perf.tick();
        self.glfw_window.swap_buffers();
        Ok(())
    }
}

struct PerfDisplayerHandler {
    old_print: Instant,
    frame_time: Duration,
    frame_count: u64,
    update_time: Duration,
    update_count: u64,
}

impl PerfDisplayerHandler {
    pub(crate) fn tick(&mut self) {
        if self.old_print.elapsed() > Duration::from_secs(1) {
            let mspu = self.update_time.as_millis() as f32 / self.update_count as f32;
            let mspf = self.frame_time.as_millis() as f32 / self.frame_count as f32;
            debug!(
                "{}FPS {:.2}MSPF / {}UPS {:.2}MSPU",
                self.frame_count, mspf, self.update_count, mspu
            );
            self.update_count = 0;
            self.update_time = Duration::ZERO;
            self.frame_count = 0;
            self.frame_time = Duration::ZERO;
            self.old_print = Instant::now();
        }
    }
}
