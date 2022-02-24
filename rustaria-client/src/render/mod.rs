use glfw::{Glfw, Window};
use tracing::debug;

use opengl_render::{ClearCommand, ClearDescriptor, OpenGlBackend};
use rustaria::api::Rustaria;

use crate::render::world_render::WorldRenderer;

mod texture_format;
mod world_render;
mod world_mesher;

pub struct RenderHandler {
    backend: OpenGlBackend,
    pub world_renderer: WorldRenderer,

    pub wireframe: bool,
}

impl RenderHandler {
    pub fn new(rsa: &Rustaria, glfw: &Glfw, window: &Window) -> eyre::Result<RenderHandler> {
        let size = window.get_size();

        debug!(target: "render", "Linking OpenGL");
        let mut opengl = OpenGlBackend::new((size.0 as u32, size.1 as u32), |procname| {
            glfw.get_proc_address_raw(procname)
        });

        debug!(target: "render", "Creating WorldRenderer");
        opengl.set_clear_command(ClearCommand {
            commands: vec![ClearDescriptor::Color(0.15, 0.15, 0.15, 1.0)],
        });
        let renderer = WorldRenderer::new(rsa,&mut opengl, window)?;
       Ok( RenderHandler {
           backend: opengl,
           world_renderer: renderer,
           wireframe: false,
       })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.backend.set_viewport_size(width, height);
        self.world_renderer.resize(width, height);
    }

    pub fn prepare_draw(&mut self) {
        self.backend.clear_frame();
    }

    pub fn draw(&mut self, pos: (f32, f32)) -> eyre::Result<()> {
        self.world_renderer.qi_pos.set_value([pos.0, pos.1]);
        self.world_renderer.draw(self.wireframe);
        Ok(())
    }
}
