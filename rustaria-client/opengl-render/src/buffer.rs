use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, Range};
use std::rc::Rc;

use image::Primitive;
use tracing::info;

use opengl::gl;
use opengl::gl::types::GLenum;

use crate::attribute::AttributeDescriptor;
use crate::raw::{RawBuffer, RawVertexBuffer};
use crate::types::GlType;

pub struct VertexBufferLayout {
    raw: Rc<RawVertexBuffer>,
    vbo: Vec<Rc<RawBuffer>>,
    index_buffer: Option<(Rc<RawBuffer>, GLenum)>,
}

impl VertexBufferLayout {
    pub fn new() -> VertexBufferLayout {
        let mut gl_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_id);
        }
        VertexBufferLayout {
            raw: Rc::new(RawVertexBuffer {
                gl_id,
            }),
            index_buffer: None,
            vbo: vec![],
        }
    }

    pub fn bind_index<T: GlType>(&mut self, buffer: &Buffer<T>) {
        match buffer.buffer_type {
            BufferType::Index(_) => {
                self.index_buffer = Some((buffer.raw.clone(), T::gl_enum()));
            }
            _ => panic!("BufferType {:?} is not an index buffer", buffer.buffer_type.get_gl()),
        }
    }

    pub fn bind_buffer<T>(&mut self, buffer: &Buffer<T>) {
        match &buffer.buffer_type {
            BufferType::Vertex(attributes) => {
                let mut attributes = attributes.clone();
                attributes.sort_by(|a1, a2| a1.index.cmp(&a2.index));
                let stride = attributes.iter().fold(0, |total, elem| total + elem.attribute_type.get_size());
                let mut offset = 0;
                unsafe {
                    for x in attributes {
                        info!("{:?} offset {} stride {}", x, offset, stride);
                        let attr_size = x.attribute_type.get_size();
                        self.bind();
                        x.attribute_type.attrib(x.index, stride as i32, offset as *const c_void);
                        gl::EnableVertexAttribArray(x.index);
                        offset += attr_size;
                    }
                }

                self.vbo.push(buffer.raw.clone());
            }

            _ => panic!("BufferType {:?} not bindable to VAO.", buffer.buffer_type.get_gl()),
        }
    }

    pub(crate) unsafe fn bind(&self) {
        if let Some((buffer, _)) = &self.index_buffer {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.gl_id);
        }
        gl::BindVertexArray(self.raw.gl_id);
    }


    pub(crate) unsafe fn draw(&self, range: Range<usize>, mode: DrawMode) {
        match &self.index_buffer {
            Some((_, index_type)) => {
                gl::DrawElements(mode.get_gl(), range.end as i32, *index_type, (range.start as i32) as *const c_void)
            }
            None => {
                gl::DrawArrays(mode.get_gl(), range.start as i32, range.end as i32);
            }
        }
    }
}

impl Default for VertexBufferLayout {
    fn default() -> Self {
        VertexBufferLayout::new()
    }
}


pub struct Buffer<T> {
    raw: Rc<RawBuffer>,
    size: usize,
    buffer_type: BufferType<T>,
    buffer_elements: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn create(
        buffer_type: BufferType<T>,
        buffer_usage: BufferUsage,
        buffer_access: BufferAccess,
        data: Option<&Vec<T>>) -> Buffer<T> {
        let target = buffer_type.get_gl();
        let usage = Self::get_usage_enum(buffer_usage, buffer_access);

        let size = data.map_or(0, |vec| vec.len() * std::mem::size_of::<T>());
        let data = data.map_or(std::ptr::null(), |vec| vec.as_ptr() as *const c_void);

        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(target, buffer);
            gl::BufferData(
                target,
                size as isize,
                data,
                usage,
            );
        }

        Buffer {
            raw: Rc::new(RawBuffer {
                gl_id: buffer,
            }),
            size,
            buffer_type,
            buffer_elements: Default::default(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.size / std::mem::size_of::<T>()
    }


    pub fn upload(&mut self, data: &[T], buffer_usage: BufferUsage, buffer_access: BufferAccess) {
        let target = self.buffer_type.get_gl();
        let size = data.len() * std::mem::size_of::<T>();
        let data = data.as_ptr() as *const c_void;

        unsafe {
            gl::BindBuffer(target, self.raw.gl_id);
            gl::BufferData(
                target,
                size as isize,
                data,
                Self::get_usage_enum(buffer_usage, buffer_access),
            );
            self.size = size;
        }
    }

    pub unsafe fn bind(&self)  {
        gl::BindBuffer(self.buffer_type.get_gl(), self.raw.gl_id);
    }

    pub fn update(&mut self, offset: usize, data: &[T]) {
        let target = self.buffer_type.get_gl();
        let offset = offset * std::mem::size_of::<T>();
        let size = data.len() * std::mem::size_of::<T>();
        let data = data.as_ptr() as *const c_void;

        if offset + size > self.size {
            panic!("{} + {} is bigger than the buffer size {}", offset, size, self.size);
        }

        unsafe {
            gl::BindBuffer(target, self.raw.gl_id);
            gl::BufferSubData(
                target,
                offset as isize,
                size as isize,
                data,
            )
        }
    }

    fn get_usage_enum(buffer_usage: BufferUsage, buffer_access: BufferAccess) -> GLenum {
        match buffer_usage {
            BufferUsage::Stream => match buffer_access {
                BufferAccess::Draw => gl::STREAM_DRAW,
                BufferAccess::Read => gl::STREAM_READ,
                BufferAccess::Copy => gl::STREAM_COPY,
            }
            BufferUsage::Static => match buffer_access {
                BufferAccess::Draw => gl::STATIC_DRAW,
                BufferAccess::Read => gl::STATIC_READ,
                BufferAccess::Copy => gl::STATIC_COPY,
            }
            BufferUsage::Dynamic => match buffer_access {
                BufferAccess::Draw => gl::DYNAMIC_DRAW,
                BufferAccess::Read => gl::DYNAMIC_READ,
                BufferAccess::Copy => gl::DYNAMIC_COPY,
            }
        }
    }
}

impl<T: GlType + Copy + IndexType> Buffer<T> {
    pub fn create_index(base_order: Vec<T>, element_size: usize, elements: usize) -> Buffer<T> {
        let data = Self::generate_index(&base_order, element_size, elements);
        Self::create(BufferType::Index(base_order), BufferUsage::Static, BufferAccess::Draw, Some(&data))
    }

    pub fn update_index(&mut self, element_size: usize, elements: usize) {
        match &self.buffer_type {
            BufferType::Index(base_order) => {
                let data = Self::generate_index(base_order, element_size, elements);
                self.upload(&data, BufferUsage::Static, BufferAccess::Draw);
            }
            _ => panic!("Not an index buffer.")
        }
    }

    fn generate_index(base_order: &[T], element_size: usize, elements: usize) -> Vec<T> {
        let mut data = Vec::new();
        for i in 0..elements {
            for index in base_order {
                data.push(index.clone().add(i * element_size));
            }
        }
        data
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

pub enum DrawMode {
    Point,
    Patches,
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
            DrawMode::Patches => gl::PATCHES,
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

pub enum BufferAccess {
    Draw,
    Read,
    Copy,
}

pub enum BufferUsage {
    Stream,
    Static,
    Dynamic,
}

pub enum BufferType<T> {
    Vertex(Vec<AttributeDescriptor>),
    Index(Vec<T>),
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

impl<T> BufferType<T> {
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
