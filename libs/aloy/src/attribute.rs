use std::ffi::c_void;

use opengl::gl;
use opengl::gl::GLenum;

pub struct FormatDescriptor {
    pub attributes: Vec<AttributeDescriptor>,
}

#[derive(Copy, Clone, Debug)]
pub struct AttributeDescriptor {
    pub index: u32,
    pub attribute_type: AttributeType,
}

impl AttributeDescriptor {
    pub fn new(index: u32, attribute_type: AttributeType) -> AttributeDescriptor {
        AttributeDescriptor {
            index,
            attribute_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AttributeType {
    Float(u8),
    Double(u8),
    Byte(u8),
    UnsignedByte(u8),
    Short(u8),
    UnsignedShort(u8),
    Int(u8),
    UnsignedInt(u8),
}

impl AttributeType {
    pub fn get_size(&self) -> usize {
        match self {
            AttributeType::Float(amount) => std::mem::size_of::<f32>() * (*amount as usize),
            AttributeType::Double(amount) => std::mem::size_of::<f64>() * (*amount as usize),
            AttributeType::Byte(amount) => std::mem::size_of::<i8>() * (*amount as usize),
            AttributeType::UnsignedByte(amount) => std::mem::size_of::<u8>() * (*amount as usize),
            AttributeType::Short(amount) => std::mem::size_of::<i16>() * (*amount as usize),
            AttributeType::UnsignedShort(amount) => std::mem::size_of::<u16>() * (*amount as usize),
            AttributeType::Int(amount) => std::mem::size_of::<i32>() * (*amount as usize),
            AttributeType::UnsignedInt(amount) => std::mem::size_of::<u32>() * (*amount as usize),
        }
    }

    pub(crate) unsafe fn attrib(&self, index: u32, stride: i32, offset: *const c_void) {
        match self {
            AttributeType::Float(amount) | AttributeType::Double(amount) => {
                let size = *amount;
                gl::VertexAttribPointer(
                    index,
                    size as i32,
                    self.get_gl_type(),
                    gl::FALSE,
                    stride,
                    offset,
                );
            }
            AttributeType::Byte(amount)
            | AttributeType::UnsignedByte(amount)
            | AttributeType::Short(amount)
            | AttributeType::UnsignedShort(amount)
            | AttributeType::Int(amount)
            | AttributeType::UnsignedInt(amount) => {
                gl::VertexAttribIPointer(index, *amount as i32, self.get_gl_type(), stride, offset);
            }
        }
    }

    pub fn get_gl_type(&self) -> GLenum {
        match self {
            AttributeType::Float(_) => gl::FLOAT,
            AttributeType::Double(_) => gl::DOUBLE,
            AttributeType::Byte(_) => gl::BYTE,
            AttributeType::UnsignedByte(_) => gl::UNSIGNED_BYTE,
            AttributeType::Short(_) => gl::SHORT,
            AttributeType::UnsignedShort(_) => gl::UNSIGNED_SHORT,
            AttributeType::Int(_) => gl::INT,
            AttributeType::UnsignedInt(_) => gl::UNSIGNED_INT,
        }
    }
}
