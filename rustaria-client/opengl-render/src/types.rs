use std::ops::Deref;
use opengl::gl;
use opengl::gl::types::GLenum;

pub trait GlType: Copy {
    fn gl_enum() -> GLenum;
}

macro_rules! gl_type_impl {
    ($($TYPE:ty => $GL:expr),*) => {
        $(impl GlType for $TYPE {
            fn gl_enum() -> GLenum {
                $GL
            }
        })*
    };
}

gl_type_impl!(
    // == Basic Types
    i8 => gl::BYTE,
    u8 => gl::UNSIGNED_BYTE,
    i16 => gl::SHORT,
    u16 => gl::UNSIGNED_SHORT,
    // == Vec Types
    // Booleans
     bool => gl::BOOL,
    [bool; 2] => gl::BOOL_VEC2,
    [bool; 3] => gl::BOOL_VEC3,
    [bool; 4] => gl::BOOL_VEC4,
    // Signed Int
     i32 => gl::INT,
    [i32; 2] => gl::INT_VEC2,
    [i32; 3] => gl::INT_VEC3,
    [i32; 4] => gl::INT_VEC4,
    // Unsigned Int
     u32 => gl::UNSIGNED_INT,
    [u32; 2] => gl::UNSIGNED_INT_VEC2,
    [u32; 3] => gl::UNSIGNED_INT_VEC3,
    [u32; 4] => gl::UNSIGNED_INT_VEC4,
    // Float
     f32 => gl::FLOAT,
    [f32; 2] => gl::FLOAT_VEC2,
    [f32; 3] => gl::FLOAT_VEC3,
    [f32; 4] => gl::FLOAT_VEC4,
    // Double
     f64 => gl::DOUBLE,
    [f64; 2] => gl::DOUBLE_VEC2,
    [f64; 3] => gl::DOUBLE_VEC3,
    [f64; 4] => gl::DOUBLE_VEC4,
    // == Matrix
    // Float
    [[f32; 2]; 2] => gl::FLOAT_MAT2,
    [[f32; 2]; 3] => gl::FLOAT_MAT2x3,
    [[f32; 2]; 4] => gl::FLOAT_MAT2x4,
    [[f32; 3]; 2] => gl::FLOAT_MAT3x2,
    [[f32; 3]; 3] => gl::FLOAT_MAT3,
    [[f32; 3]; 4] => gl::FLOAT_MAT3x4,
    [[f32; 4]; 2] => gl::FLOAT_MAT4x2,
    [[f32; 4]; 3] => gl::FLOAT_MAT4x3,
    [[f32; 4]; 4] => gl::FLOAT_MAT4,
    // Double
    [[f64; 2]; 2] => gl::DOUBLE_MAT2,
    [[f64; 2]; 3] => gl::DOUBLE_MAT2x3,
    [[f64; 2]; 4] => gl::DOUBLE_MAT2x4,
    [[f64; 3]; 2] => gl::DOUBLE_MAT3x2,
    [[f64; 3]; 3] => gl::DOUBLE_MAT3,
    [[f64; 3]; 4] => gl::DOUBLE_MAT3x4,
    [[f64; 4]; 2] => gl::DOUBLE_MAT4x2,
    [[f64; 4]; 3] => gl::DOUBLE_MAT4x3,
    [[f64; 4]; 4] => gl::DOUBLE_MAT4
);
