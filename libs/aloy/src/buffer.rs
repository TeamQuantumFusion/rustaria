use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Range;
use std::rc::Rc;
use std::time::Instant;

use tracing::{debug, info};

use opengl::gl;
use opengl::gl::GLenum;
use eyre::{Report, Result};

use crate::attribute::AttributeDescriptor;
use crate::raw::{RawBuffer, RawVertexBuffer};
use crate::types::GlType;

pub struct VertexPipeline {
    raw: Rc<RawVertexBuffer>,
    vbo: Vec<Rc<RawBuffer>>,
    index_buffer: Option<(Rc<RawBuffer>, GLenum)>,
}

impl VertexPipeline {
    pub fn new() -> VertexPipeline {
        let mut gl_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_id);
        }
        VertexPipeline {
            raw: Rc::new(RawVertexBuffer::new(gl_id)),
            index_buffer: None,
            vbo: vec![],
        }
    }

    pub fn bind_buffer<T>(&mut self, buffer: &Buffer<T>) {
        match &buffer.buffer_type {
            BufferType::Vertex(attributes) => {
                let mut attributes = attributes.clone();
                attributes.sort_by(|a1, a2| a1.index.cmp(&a2.index));
                let stride = attributes
                    .iter()
                    .fold(0, |total, elem| total + elem.attribute_type.get_size());
                let mut offset = 0;
                unsafe {
                    for x in attributes {
                        debug!("{:?} offset {} stride {}", x, offset, stride);
                        let attr_size = x.attribute_type.get_size();
                        self.bind();
                        x.attribute_type
                            .attrib(x.index, stride as i32, offset as *const c_void);
                        gl::EnableVertexAttribArray(x.index);
                        offset += attr_size;
                    }
                }
                self.vbo.push(buffer.raw.clone());
            }
            BufferType::Index(ty) => {
                self.index_buffer = Some((buffer.raw.clone(), *ty));
            }
            _ => panic!(
                "BufferType {:?} not bindable to VAO.",
                buffer.buffer_type.get_gl()
            ),
        }
    }

    pub(crate) unsafe fn bind(&self) {
        if let Some((buffer, _)) = &self.index_buffer {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id());
        }
        gl::BindVertexArray(self.raw.id());
    }

    pub(crate) unsafe fn draw(&self, range: Range<usize>, mode: DrawMode) {
        match &self.index_buffer {
            Some((_, index_type)) => gl::DrawElements(
                mode.get_gl(),
                range.end as i32,
                *index_type,
                (range.start as i32) as *const c_void,
            ),
            None => {
                gl::DrawArrays(mode.get_gl(), range.start as i32, range.end as i32);
            }
        }
    }
}

impl Default for VertexPipeline {
    fn default() -> Self {
        VertexPipeline::new()
    }
}

pub struct Buffer<T> {
    raw: Rc<RawBuffer>,
    size: usize,
    elements: usize,
    buffer_type: BufferType,
    buffer_usage: GLenum,
    ty: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn create(
        buffer_type: BufferType,
        buffer_usage: BufferUsage,
        buffer_access: BufferAccess,
        size: usize,
    ) -> Buffer<T> {
        let target = buffer_type.get_gl();
        let usage = Self::get_usage_enum(buffer_usage, buffer_access);
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(target, buffer);
            gl::BufferData(target, size as isize, std::ptr::null(), usage);
        }

        Buffer {
            raw: Rc::new(RawBuffer::new(buffer)),
            size,
            elements: 0,
            buffer_type,
            buffer_usage: usage,
            ty: Default::default(),
        }
    }

    pub fn get_elements(&self) -> usize {
        self.elements
    }

    pub fn update(&mut self, offset: usize, data: &[T]) -> Result<()>{
        let target = self.buffer_type.get_gl();
        let offset = offset * std::mem::size_of::<T>();
        let size = data.len() * std::mem::size_of::<T>();
        let data = data.as_ptr() as *const c_void;

        unsafe {
            let required_size = offset + size;
            if required_size > self.size {
                return Err(Report::msg("Buffer is too small."));
            }

            gl::BindBuffer(target, self.raw.id());
            gl::BufferSubData(target, offset as isize, size as isize, data)
        }

        Ok(())
    }

    pub fn set(&mut self, data: &[T]) -> Result<()> {
        self.elements = data.len();
        let target = self.buffer_type.get_gl();
        let size = data.len() * std::mem::size_of::<T>();
        let data = data.as_ptr() as *const c_void;

        unsafe {
            gl::BindBuffer(target, self.raw.id());
            if size > self.size {
                gl::BufferData(target, size as isize, data, self.buffer_usage);
                info!("EXPAND {}", size);
                self.size = size;
            } else {
                gl::BufferSubData(target, 0, size as isize, data)
            }
        }

        Ok(())
    }

    fn get_usage_enum(buffer_usage: BufferUsage, buffer_access: BufferAccess) -> GLenum {
        match buffer_usage {
            BufferUsage::Stream => match buffer_access {
                BufferAccess::Draw => gl::STREAM_DRAW,
                BufferAccess::Read => gl::STREAM_READ,
                BufferAccess::Copy => gl::STREAM_COPY,
            },
            BufferUsage::Static => match buffer_access {
                BufferAccess::Draw => gl::STATIC_DRAW,
                BufferAccess::Read => gl::STATIC_READ,
                BufferAccess::Copy => gl::STATIC_COPY,
            },
            BufferUsage::Dynamic => match buffer_access {
                BufferAccess::Draw => gl::DYNAMIC_DRAW,
                BufferAccess::Read => gl::DYNAMIC_READ,
                BufferAccess::Copy => gl::DYNAMIC_COPY,
            },
        }
    }
}

pub trait IndexType {
    fn add(self, size: usize) -> Self;
}

impl IndexType for u16 {
    fn add(self, size: usize) -> Self {
        self + size as u16
    }
}

impl IndexType for u32 {
    fn add(self, size: usize) -> Self {
        self + size as u32
    }
}

pub enum DrawMode {
    Point,
    Line,
    LineLoop,
    LineStrip,
    LineStripAdjacency,
    LineAdjacency,
    Triangle,
    TriangleFan,
    TriangleStrip,
    TriangleStripAdjacency,
    TriangleAdjacency,
}

impl DrawMode {
    pub(crate) fn get_gl(&self) -> GLenum {
        match self {
            DrawMode::Point => gl::POINTS,
            DrawMode::Line => gl::LINES,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::LineAdjacency => gl::LINES_ADJACENCY,
            DrawMode::Triangle => gl::TRIANGLES,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::TriangleAdjacency => gl::TRIANGLES_ADJACENCY,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BufferAccess {
    Draw,
    Read,
    Copy,
}

#[derive(Clone, Copy)]
pub enum BufferUsage {
    Stream,
    Static,
    Dynamic,
}

pub enum BufferType {
    Vertex(Vec<AttributeDescriptor>),
    Index(GLenum),
    CopyRead,
    CopyWrite,
    PixelUnpack,
    PixelPack,
    Query,
    Texture,
    TransformFeedback,
    Uniform,
    DrawIndirect,
    AtomicCounter,
    DispatchIndirect,
    ShaderStorage,
}

impl BufferType {
    pub fn index<T: GlType>() -> BufferType {
        BufferType::Index(T::gl_enum())
    }
}

impl BufferType {
    pub(crate) fn get_gl(&self) -> GLenum {
        match self {
            BufferType::Vertex(..) => gl::ARRAY_BUFFER,
            BufferType::Index(..) => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::CopyRead => gl::COPY_READ_BUFFER,
            BufferType::CopyWrite => gl::COPY_WRITE_BUFFER,
            BufferType::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            BufferType::PixelPack => gl::PIXEL_PACK_BUFFER,
            BufferType::Query => gl::QUERY_BUFFER,
            BufferType::Texture => gl::TEXTURE_BUFFER,
            BufferType::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferType::Uniform => gl::UNIFORM_BUFFER,
            BufferType::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            BufferType::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            BufferType::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            BufferType::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    BufferTooSmall,
}
