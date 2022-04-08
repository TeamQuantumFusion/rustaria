use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Window, WindowEvent};

use crate::vertex::VertexBuilder;

pub mod ty;
pub mod vertex;

pub trait RenderBackend {
    fn new(glfw: &mut Glfw, window: &mut Window) -> Self;
    fn resize(&mut self, size: (u32, u32));
    fn submit<V: Clone>(&mut self, identifier: RenderLayerIdentifier, buffer: VertexBuilder<V>);
    fn draw(&mut self);
}

/// An identifier for which render layer we are targeting.
pub struct RenderLayerIdentifier {
    pub color: bool,
    pub texture: bool,
    pub persistence: RenderLayerPersistence,
}

pub enum RenderLayerPersistence {
    Static,
    Dynamic,
}

pub struct RenderHandler<B: RenderBackend> {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    backend: B,
}

impl<B: RenderBackend> RenderHandler<B> {
    pub fn new() -> RenderHandler<B> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(1920 / 2, 1080 / 2, "Rustaria", glfw::WindowMode::Windowed)
            .unwrap();

        window.set_all_polling(true);

        let backend = B::new(&mut glfw, &mut window);
        RenderHandler {
            glfw,
            window,
            events,
            backend,
        }
    }

    pub fn alive(&self) -> bool {
        !self.window.should_close()
    }

    pub fn poll<F: Fn(WindowEvent)>(&mut self, func: F) {
        self.glfw.poll_events();
        while let Ok(event) = self.events.try_recv() {
            func(event.1);
        }
    }
    pub fn submit<V: Clone>(
        &mut self,
        identifier: RenderLayerIdentifier,
        buffer: VertexBuilder<V>,
    ) {
        self.backend.submit(identifier, buffer);
    }

    pub fn draw(&mut self) {
        self.backend.draw();
        self.window.swap_buffers();
    }
}

impl<B: RenderBackend> Default for RenderHandler<B> {
    fn default() -> Self {
        Self::new()
    }
}
