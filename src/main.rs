use eyre::Result;
use time::macros::format_description;
use tracing::info;
use tracing_subscriber::{fmt::time::UtcTime, prelude::*, EnvFilter};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

fn main() -> Result<()> {
    init();
    info!("rustaria v{}", env!("CARGO_PKG_VERSION"));

    let evloop = EventLoop::new();
    let mut window = WindowBuilder::new().build(&evloop)?;

    evloop.run(move |event, target, cf| event_loop(&mut window, event, target, cf))
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
    event: Event<()>,
    _target: &EventLoopWindowTarget<()>,
    cf: &mut ControlFlow,
) {
    *cf = ControlFlow::Poll;

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
            _ => {}
        },
        _ => {}
    }
}
