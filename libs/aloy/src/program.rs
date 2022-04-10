use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Range;
use std::os::raw::c_char;
use std::rc::Rc;
use std::sync::RwLock;

use opengl::gl;
use opengl::gl::{GLenum, GLint, GLuint};

use crate::attribute::FormatDescriptor;
use crate::buffer::{DrawMode, VertexPipeline};
use crate::raw::RawProgram;
use crate::uniform::{Uniform, UniformError, UniformType, UniformValueBinder};

pub struct Program {
    id: RawProgram,
    program_uniforms: HashMap<String, (GLenum, i32)>,
    uniforms: Vec<(i32, Rc<RwLock<UniformValueBinder>>)>,
}

impl Program {
    pub fn new(vertex_code: String, fragment_code: String) -> Program {
        let id = unsafe {
            let id = gl::CreateProgram();
            let vertex_shader = Self::create_shader(vertex_code, ShaderType::Vertex);
            gl::AttachShader(id, vertex_shader);

            let fragment_shader = Self::create_shader(fragment_code, ShaderType::Fragment);
            gl::AttachShader(id, fragment_shader);

            gl::LinkProgram(id);

            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != 1 {
                let mut len: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let info_log = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut c_char,
                );

                panic!(
                    "Program Linking error {}",
                    info_log.to_string_lossy().into_owned()
                );
            }

            // OpenGL spec tells you can do this and they will actually get deleted when the program is done.
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            id
        };

        // get uniforms
        let mut program_uniforms = HashMap::new();
        unsafe {
            let mut active_uniforms = 0;
            gl::GetProgramiv(id, gl::ACTIVE_UNIFORMS, &mut active_uniforms);

            let name_data = CString::from_vec_unchecked(vec![b' '; 128]);
            for index in 0..active_uniforms {
                let mut size = 0;
                let mut ty = 0;
                let mut name_length = 0;
                gl::GetActiveUniform(
                    id,
                    index as u32,
                    128,
                    &mut name_length,
                    &mut size,
                    &mut ty,
                    name_data.as_ptr() as *mut c_char,
                );
                let bytes = name_data.to_bytes();
                let name = String::from_utf8_lossy(&bytes[0..name_length as usize]);
                program_uniforms.insert(name.to_string(), (ty, index));
            }
        }

        Program {
            id: RawProgram::new(id),
            program_uniforms,
            uniforms: vec![],
        }
    }

    pub fn get_uniform<T: UniformType>(&mut self, name: &str) -> Result<Uniform<T>, UniformError> {
        let (uniform_type, index) = self
            .program_uniforms
            .get(name)
            .ok_or(UniformError::UniformDoesNotExist)?;
        if T::gl_enum() != *uniform_type {
            return Err(UniformError::UniformTypeMismatch(*uniform_type));
        }

        let value = self
            .uniforms
            .iter()
            .find(|(id, _)| *id == *index)
            .map(|value| value.1.clone())
            .unwrap_or_else(|| {
                let uniform_binder = T::get_binder(T::default());
                let value = Rc::new(RwLock::new(uniform_binder));
                self.uniforms.push((*index, value.clone()));
                value
            });

        Ok(Uniform::new(value))
    }

    pub(crate) unsafe fn create_shader(code: String, shader_type: ShaderType) -> GLuint {
        let id = gl::CreateShader(shader_type.get_gl());
        let string = CString::new(code).unwrap();

        gl::ShaderSource(id, 1, &string.as_ptr(), std::ptr::null());
        gl::CompileShader(id);

        let mut success = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success != 1 {
            let mut len: GLint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let info_log = CString::from_vec_unchecked(vec![b' '; len as usize]);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                info_log.as_ptr() as *mut c_char,
            );
            panic!(
                "Shader Compile Error {}",
                info_log.to_string_lossy().into_owned()
            );
        }

        id
    }

    pub fn draw(&self, layout: &VertexPipeline, range: Range<usize>, mode: DrawMode) {
        unsafe {
            if !range.is_empty() {
                gl::UseProgram(self.id.id());
                for (index, uniform) in &self.uniforms {
                    uniform.read().unwrap().bind(*index)
                }
                layout.bind();
                layout.draw(range, mode);
            }
        }
    }
}

pub struct ProgramDescriptor {
    pub vertex_code: String,
    pub fragment_code: String,
    pub attributes: FormatDescriptor,
}

pub enum ShaderType {
    Vertex,
    Geometry,
    Fragment,
    Compute,
}

impl ShaderType {
    pub(crate) fn get_gl(&self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        }
    }
}
