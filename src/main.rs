use eyre::Result;
use time::macros::format_description;
use tracing::{error, info};
use tracing_subscriber::{fmt::time::UtcTime, prelude::*, EnvFilter};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use crate::{plugin::PluginLoader, renderer::Renderer};

mod api_impl;
mod plugin;
mod renderer;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    info!("rustaria v{}", env!("CARGO_PKG_VERSION"));

    let plugins = PluginLoader::new();
    plugins.load_plugin_from_bytes(include_bytes!(
        "/home/leocth/coding/rust/rustaria/target/wasm32-wasi/debug/rustaria_core.wasm"
    ))?;
    Ok(())

    // let evloop = EventLoop::new();
    // let mut window = WindowBuilder::new().build(&evloop)?;
    // let mut renderer = Renderer::new(&window).await;

    // evloop.run(move |event, target, cf| event_loop(&mut window, &mut renderer, event, target, cf))
}

fn init() {
    let timer = UtcTime::new(format_description!(
        "[hour]:[minute]:[second].[subsecond digits:3]"
    ));
    let format = tracing_subscriber::fmt::format()
        .with_timer(timer)
        .compact();
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(format);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("`info` is not a valid EnvFilter... what?");
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
}

fn event_loop(
    window: &mut Window,
    renderer: &mut Renderer,
    event: Event<()>,
    _target: &EventLoopWindowTarget<()>,
    cf: &mut ControlFlow,
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
                Ok(_) => {}
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
