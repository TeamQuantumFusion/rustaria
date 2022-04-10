use std::sync::mpsc::Receiver;

use glfw::WindowEvent::FramebufferSize;
use glfw::{
    Context, Glfw, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint, WindowMode,
};

use aloy::vertex::VertexBuilder;
use aloy::{ClearCommand, ClearDescriptor, OpenGlBackend};
use renderer::pipeline::RenderPipeline;
use rustaria_util::{ContextCompat, Result};
use ty::Pos;

use crate::renderer::RenderingHandler;
use crate::ty::Viewport;

pub mod renderer;
pub mod ty;
pub mod world_drawer;

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

/// Ok so. I did not want to make a crate for glfw, window, backend and event receiver
/// because it would become too concerning with depenencies.
/// and because i could not come up with a name for it.
///
/// # Why is this a Battlecruiser that is probably operational.
/// Well i could not think of a name for this either so i named it after a StarCraft carrier ship.
/// It was a choice between the battlecruiser and the carrier (zerg cringe).
/// The reason i chose the Battle Cruiser is because my dad joked about the "Battlecruiser operational!"
/// quote that the captain does when it spawns.
/// But yes. Battlecruiser operational!
///
/// # What is a battlecruiser?
/// If you are not a boomer like me, go here https://starcraft.fandom.com/wiki/Battlecruiser_(StarCraft)
///
/// # I'm leo and I want to rename this.
/// If you do that im renaming `mooncake` to `lua_runtime_reference_method_wrapper_macro_crate`
pub struct BattleCruiser {
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    backend: OpenGlBackend,
    glfw: Glfw,
}

impl BattleCruiser {
    // If you ask me why this is not called new. Just go to its usages and check how it looks.
    pub fn operational() -> Result<BattleCruiser> {
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
        window.set_scroll_polling(true);
        window.set_size_polling(true);
        window.set_framebuffer_size_polling(true);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        let mut backend = OpenGlBackend::new(size, |procname| glfw.get_proc_address_raw(procname));
        window.set_size(900, 600);
        backend.set_clear_command(ClearCommand {
            commands: vec![ClearDescriptor::Color(0.15, 0.15, 0.15, 1.0)],
        });
        Ok(BattleCruiser {
            window,
            events,
            backend,
            glfw,
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

    pub fn draw(&mut self, renderer: &mut RenderingHandler, view: &Viewport) {
        self.backend.clear_frame();
        renderer.draw(view);
        self.window.swap_buffers();
    }
}
