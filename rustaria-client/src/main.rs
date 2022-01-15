use std::path::PathBuf;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use crate::renderer::Renderer;

pub mod renderer;

use eyre::{eyre, Result};
use tracing::{error, info};
use rustaria::api;
use rustaria::api::LuaRuntime;
use rustaria::chunk::Chunk;
use rustaria::player::Player;
use rustaria::registry::Tag;
use rustaria::world::World;

#[tokio::main]
async fn main() -> Result<()> {

    rustaria::init_console(true)?;

    info!("Rustaria Dedicated Server v{}", env!("CARGO_PKG_VERSION"));
    let runtime = LuaRuntime::new();
    let mut api = api::launch_rustaria_api(PathBuf::from("./plugins"), &runtime).await?;

    // create runtime
    let air_tile = api
        .tiles
        .get_id(&Tag::parse("rustaria-core:air")?)
        .ok_or_else(|| eyre!("Could not find air tile"))?;
    let air_wall = api
        .walls
        .get_id(&Tag::parse("rustaria-core:air")?)
        .ok_or_else(|| eyre!("Could not find air wall"))?;
    let empty_chunk = Chunk::new(&api, air_tile, air_wall)
        .ok_or_else(|| eyre!("Could not create empty chunk"))?;
    let mut world = World::new(
        (2, 2),
        vec![empty_chunk, empty_chunk, empty_chunk, empty_chunk],
    )?;

    info!("f");

    world.player_join(Player::new(0.0, 0.0, "dev".to_string()));
    let evloop = EventLoop::new();
    let mut window = WindowBuilder::new().build(&evloop)?;

    let mut renderer = Renderer::new(&window, &mut api).await;
    info!("f");

    let mut profiler = Profiler {
        last_fps: Instant::now(),
        fps: 0,
    };

    evloop.run(move |event, target, cf| {
        event_loop(&mut window, &mut renderer, event, target, cf, &mut profiler)
    });
}

fn event_loop(
    window: &mut Window,
    renderer: &mut Renderer,
    event: Event<()>,
    _target: &EventLoopWindowTarget<()>,
    cf: &mut ControlFlow,
    profiler: &mut Profiler,
) {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *cf = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => renderer.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            renderer.update();
            match renderer.render() {
                Ok(_) => {
                    profiler.fps += 1;
                    if profiler.last_fps.elapsed().as_millis() > 1000 {
                        println!("FPS: {}", profiler.fps);

                        profiler.fps = 0;
                        profiler.last_fps = Instant::now();
                    }
                }
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *cf = ControlFlow::Exit,
                Err(e) => error!("{:?}", e),
            }
        }
        _ => {}
    }
}

pub struct Profiler {
    last_fps: Instant,
    fps: u128,
}
