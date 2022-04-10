use std::sync::mpsc::Receiver;
use std::time::Instant;

use glfw::{Context, Glfw, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint, WindowMode};
use glfw::WindowEvent::FramebufferSize;

use aloy::{ClearCommand, ClearDescriptor, OpenGlBackend, OpenGlFeature};
use aloy::vertex::VertexBuilder;
use renderer::pipeline::DrawPipeline;
use rustaria::api::Api;
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria_util::{debug, ContextCompat, Result};
use ty::{Color, Pos};

use crate::profiler::Profiler;
use crate::renderer::WorldRenderer;
use crate::ty::Viewport;

pub mod profiler;
pub mod renderer;
pub mod ty;

/// An identifier for which render layer we are targeting.
#[derive(Debug)]
pub struct RenderLayerIdentifier {
    pub color: bool,
    pub texture: bool,
    pub persistence: RenderLayerStability,
}

#[derive(Debug)]
pub enum RenderLayerStability {
    Stable,
    Erratic,
}

pub struct RenderHandler {
   profiler: Profiler,
    pub window: Window,
    events: Receiver<(f64, WindowEvent)>,
    backend: OpenGlBackend,
    // needs to be last or the context will drop
    glfw: Glfw,
}

impl RenderHandler {
    pub fn new() -> Result<RenderHandler> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlDebugContext(false));
        glfw.window_hint(WindowHint::ContextVersion(4, 5));
        let size = (1920 / 2, 1080 / 2);
        let (mut window, events) = glfw
            .create_window(size.0, size.1, "Rustaria", WindowMode::Windowed)
            .wrap_err("Could not create window")?;

        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_size_polling(true);
        window.set_framebuffer_size_polling(true);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        let mut backend = OpenGlBackend::new(size, |procname| glfw.get_proc_address_raw(procname));
        backend.set_clear_command(ClearCommand {
            commands: vec![ClearDescriptor::Color(0.1, 0.1, 0.1, 1.0)],
        });
        backend.enable(OpenGlFeature::Alpha);

        Ok(RenderHandler {
            profiler: Profiler::new(),
            backend,
            glfw,
            window,
            events,
        })
    }

    pub fn alive(&self) -> bool {
        !self.window.should_close()
    }

    pub fn poll<F: FnMut(WindowEvent)>(&mut self, mut func: F) {
        self.glfw.poll_events();
        while let Ok((_, event)) = self.events.try_recv() {
            if let FramebufferSize(width, height) = event {
                let width = width as u32;
                let height = height as u32;
                self.backend.set_viewport_size(width, height);
            }
            func(event);
        }
    }

    pub fn start_frame(&mut self) -> &mut Profiler{
        self.backend.clear_frame();
        self.profiler.start_frame();
        &mut self.profiler
    }

    pub fn stop_frame(&mut self) {
        self.profiler.end_frame();
        self.window.swap_buffers();
    }
}

pub trait LayerSubmitter<V: Clone> {
    fn submit(&mut self, buffer: VertexBuilder<V>, stability: RenderLayerStability) -> Result<()>;
}