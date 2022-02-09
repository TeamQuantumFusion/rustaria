use std::ffi::c_void;
use std::sync::mpsc::Receiver;

use glfw::{Context, FlushedMessages, Glfw, Window, WindowEvent};
use tracing::info;

use opengl::gl;
use opengl::gl::types::{GLbitfield, GLenum};

use crate::texture::{Sampler2d, Texture, USampler2d};
use crate::util::RustGlEnum;

#[macro_use]
mod util;
pub mod program;
pub mod attribute;
pub mod buffer;
pub mod uniform;
pub mod texture;
mod raw;
mod types;

pub struct OpenGlBackend {
    clear_bit: GLbitfield,
    viewport_size: (u32, u32),
}

impl OpenGlBackend {
    pub fn new<F: FnMut(&'static str) -> *const c_void>(viewport_size: (u32, u32), loader_func: F) -> OpenGlBackend {
        gl::load_with(loader_func);
        OpenGlBackend {
            clear_bit: 0,
            viewport_size,
        }
    }

    pub fn create_sampler(&mut self, unit: u8, texture: &Texture) -> Sampler2d {
        let gl_unit = gl::TEXTURE0 + unit as u32;
        unsafe {
            gl::ActiveTexture(gl_unit);
            texture.bind();
        }
        Sampler2d {
            unit
        }
    }

    pub fn enable(&mut self, feature: OpenGlFeature) {
        unsafe {
            match feature {
                OpenGlFeature::Alpha => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
            }
        }
    }

    pub fn set_clear_command(&mut self, command: ClearCommand) {
        self.clear_bit = 0;
        unsafe {
            for x in command.commands {
                match x {
                    ClearDescriptor::Color(r, g, b, a) => {
                        gl::ClearColor(r, g, b, a);
                        self.clear_bit |= gl::COLOR_BUFFER_BIT;
                    }
                    ClearDescriptor::Depth => {
                        self.clear_bit |= gl::DEPTH_BUFFER_BIT;
                    }
                }
            }
        }
    }

    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.viewport_size = (width, height);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn clear_frame(&mut self) {
        unsafe {
            gl::Clear(self.clear_bit);
        }
    }
}

// Clear stuff
pub struct ClearCommand {
    pub commands: Vec<ClearDescriptor>,
}

pub enum ClearDescriptor {
    Color(f32, f32, f32, f32),
    Depth,
}

pub enum OpenGlFeature {
    Alpha
}


