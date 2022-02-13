use tracing::debug;

use opengl::gl;
use opengl::gl::GLuint;

pub(crate) struct RawProgram {
    pub(crate) gl_id: GLuint,
}

impl Drop for RawProgram {
    fn drop(&mut self) {
        unsafe {
            debug!(target: "opengl", "Dropped Program {}", self.gl_id);
            gl::DeleteProgram(self.gl_id);
        }
    }
}

pub(crate) struct RawBuffer {
    pub(crate) gl_id: GLuint,
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        unsafe {
            debug!(target: "opengl", "Dropped Buffer {}", self.gl_id);
            gl::DeleteBuffers(1, &self.gl_id);
        }
    }
}

pub(crate) struct RawVertexBuffer {
    pub(crate) gl_id: GLuint,
}

impl Drop for RawVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            debug!(target: "opengl", "Dropped VAO {}", self.gl_id);
            gl::DeleteVertexArrays(1, &self.gl_id);
        }
    }
}

pub(crate) struct RawTexture {
    pub(crate) gl_id: GLuint,
}

impl Drop for RawTexture {
    fn drop(&mut self) {
        unsafe {
            debug!(target: "opengl", "Dropped Texture {}", self.gl_id);
            gl::DeleteTextures(1, &self.gl_id);
        }
    }
}
