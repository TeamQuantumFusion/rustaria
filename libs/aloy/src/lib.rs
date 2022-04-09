use std::ffi::{c_void, CStr, CString};
use tracing::{debug, error, info, warn};

use opengl::gl;
use opengl::gl::{GLbitfield, GLchar, GLenum, GLsizei, GLuint};

use crate::texture::{Sampler2d, Texture};
#[macro_use]
mod util;
pub mod atlas;
pub mod attribute;
pub mod buffer;
pub mod program;
mod raw;
pub mod texture;
mod types;
pub mod uniform;

pub struct OpenGlBackend {
    clear_bit: GLbitfield,
    viewport_size: (u32, u32),
}

impl OpenGlBackend {
    pub fn new<F: FnMut(&'static str) -> *const c_void>(
        viewport_size: (u32, u32),
        mut loader_func: F,
    ) -> OpenGlBackend {
        unsafe {
            gl::load_gl(&mut loader_func);
            gl::DebugMessageCallback(error_callback, std::ptr::null());
        }

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
        Sampler2d { unit }
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
            debug!("Resize {}x{}", width, height);
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn clear_frame(&mut self) {
        unsafe {
            gl::Clear(self.clear_bit);
        }
    }
}

extern "system" fn error_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut std::os::raw::c_void,
) {
    unsafe {
        let cstr = CStr::from_ptr(message);
        match severity {
            gl::DEBUG_SEVERITY_LOW => info!(target: "opengl-driver", "{:?}", cstr),
            gl::DEBUG_SEVERITY_MEDIUM => warn!(target: "opengl-driver", "{:?}", cstr),
            gl::DEBUG_SEVERITY_HIGH => error!(target: "opengl-driver", "{:?}", cstr),
            _ => debug!(target: "opengl-driver", "{:?}", cstr),
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
    Alpha,
}
