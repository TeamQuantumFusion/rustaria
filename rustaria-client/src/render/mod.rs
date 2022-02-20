use glfw::{Glfw, Window};

use opengl_render::{ClearCommand, ClearDescriptor, OpenGlBackend};

use crate::render::world_render::WorldRenderer;

mod texture_format;
mod world_render;

pub struct RenderHandler {
    backend: OpenGlBackend,
    pub world_renderer: WorldRenderer,

    pub wireframe: bool,
}

impl RenderHandler {
    pub fn new(glfw: &Glfw, window: &Window) -> RenderHandler {
        let size = window.get_size();
        let mut opengl = OpenGlBackend::new((size.0 as u32, size.1 as u32), |procname| {
            glfw.get_proc_address_raw(procname)
        });
        opengl.set_clear_command(ClearCommand {
            commands: vec![ClearDescriptor::Color(0.2, 0.2, 0.2, 1.0)],
        });
        let renderer = WorldRenderer::new(&mut opengl, window);
        RenderHandler {
            backend: opengl,
            world_renderer: renderer,
            wireframe: false,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.backend.set_viewport_size(width, height);
        self.world_renderer.resize(width, height);
    }

    pub fn draw(&mut self, pos: (f32, f32)) -> eyre::Result<()> {
        self.world_renderer.qi_pos.set_value([pos.0, pos.1]);
        self.backend.clear_frame();
        self.world_renderer.draw(self.wireframe);
        Ok(())
    }
}
