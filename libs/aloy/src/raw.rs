use rustaria_util::debug;

use opengl::gl;
use opengl::gl::GLuint;

macro_rules! new {
    ($TYPE:ty : $NAME:literal => $DEL_METHOD:ident) => {
        impl $TYPE {
            pub fn new(gl_id: GLuint) -> Self {
                debug!(target: "opengl", "Created {} {}", $NAME, gl_id);
                Self {
                    gl_id
                }
            }

            pub fn id(&self) -> GLuint {
                self.gl_id
            }
        }

        impl Drop for $TYPE {
            fn drop(&mut self) {
                unsafe {
                    debug!(target: "opengl", "Dropped {} {}", $NAME, self.gl_id);
                    $DEL_METHOD(self.gl_id);
                }
            }
        }
    };
}

pub(crate) struct RawProgram {
    gl_id: GLuint,
}

pub(crate) struct RawBuffer {
    gl_id: GLuint,
}

pub(crate) struct RawVertexBuffer {
    gl_id: GLuint,
}

pub(crate) struct RawTexture {
    gl_id: GLuint,
}
new!(RawProgram : "Program" => del_program);
new!(RawBuffer : "Buffer" => del_buffer);
new!(RawVertexBuffer : "VAO" => del_vao);
new!(RawTexture : "Texture" => del_texture);

unsafe fn del_program(id: GLuint) {
    gl::DeleteProgram(id);
}

unsafe fn del_buffer(id: GLuint) {
    gl::DeleteBuffers(1, &id);
}

unsafe fn del_vao(id: GLuint) {
    gl::DeleteVertexArrays(1, &id);
}

unsafe fn del_texture(id: GLuint) {
    gl::DeleteTextures(1, &id);
}
