use std::ops::AddAssign;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use eyre::Result;
use glfw::{Action, Context, Glfw, Key, Modifiers, SwapInterval, Window, WindowEvent};
use structopt::StructOpt;
use tracing::{debug, info};

use rustaria::player::Player;
use rustaria::UPS;

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

    let mut client = RustariaClient::new();
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
    glfw: Glfw,
    glfw_window: Window,
    glfw_events: Receiver<(f64, WindowEvent)>,

    player: Player,
    zoom: f32,
    // this is bad
    w: bool,
    a: bool,
    s: bool,
    d: bool,

    render: RenderHandler,
    perf: PerfDisplayerHandler,
}

impl RustariaClient {
    pub fn new() -> RustariaClient {
        let title = format!("Rustaria Client {}", env!("CARGO_PKG_VERSION"));
        info!("{}", title);

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

        let render = RenderHandler::new(&glfw, &glfw_window);
        RustariaClient {
            glfw,
            glfw_window,
            glfw_events,
            player: Player {
                pos: (0.0, 0.0),
                vel: (0.0, 0.0),
            },
            zoom: 1.0,
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
        }
    }

    pub fn running(&self) -> bool {
        !self.glfw_window.should_close()
    }

    pub fn tick(&mut self) {
        let update_time = Instant::now();
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
                    self.zoom -= y as f32 * 0.01;
                    self.render.world_renderer.qi_u_zoom.set_value(self.zoom);
                }
                WindowEvent::Key(Key::Q, _, Action::Press, DEBUG_MOD) => {
                    self.glfw_window.set_should_close(true)
                }
                WindowEvent::Key(Key::W, _, Action::Press, DEBUG_MOD) => {
                    self.render.wireframe = !self.render.wireframe
                }
                WindowEvent::Key(Key::W, _, action, _) => self.w = action != Action::Release,
                WindowEvent::Key(Key::A, _, action, _) => self.a = action != Action::Release,
                WindowEvent::Key(Key::S, _, action, _) => self.s = action != Action::Release,
                WindowEvent::Key(Key::D, _, action, _) => self.d = action != Action::Release,
                _ => {}
            }
        }

        let draw_time = Instant::now();
        let pos = (
            self.player.pos.0 + (self.player.vel.0 * delta),
            self.player.pos.1 + (self.player.vel.1 * delta),
        );
        self.render.draw(pos)?;
        self.perf.frame_time.add_assign(draw_time.elapsed());
        self.perf.frame_count += 1;
        self.perf.tick();
        self.glfw_window.swap_buffers();
        Ok(())
    }
}

impl Default for RustariaClient {
    fn default() -> Self {
        Self::new()
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
