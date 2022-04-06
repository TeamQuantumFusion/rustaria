use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;

use crate::texture::{Sampler2d, USampler2d};
use opengl::gl;
use opengl::gl::GLenum;

use crate::types::GlType;

#[derive(Debug)]
pub enum UniformError {
    UniformDoesNotExist,
    UniformTypeMismatch(GLenum),
}

pub struct Uniform<T: UniformType> {
    uniform_value: Rc<RwLock<UniformValueBinder>>,
    uniform_location: i32,
    uniform_type: PhantomData<T>,
}

impl<T: UniformType> Uniform<T> {
    pub(crate) fn new(index: i32, value: Rc<RwLock<UniformValueBinder>>) -> Uniform<T> {
        Uniform {
            uniform_value: value,
            uniform_location: index,
            uniform_type: Default::default(),
        }
    }

    pub fn set_value(&mut self, value: T) {
        *self.uniform_value.write().unwrap() = value.get_binder();
    }
}

pub enum UniformValueBinder {
    Float(f32),
    Double(f64),
    Int(i32),
    UInt(u32),
    FVec2([f32; 2]),
    FVec3([f32; 3]),
    FVec4([f32; 4]),
    DVec2([f64; 2]),
    DVec3([f64; 3]),
    DVec4([f64; 4]),
    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),
    UIVec2([u32; 2]),
    UIVec3([u32; 3]),
    UIVec4([u32; 4]),
    Sampler2D(Sampler2d),
    USampler2D(USampler2d),
}

impl UniformValueBinder {
    pub(crate) unsafe fn bind(&self, location: i32) {
        use UniformValueBinder::*;
        match self {
            Float(v0) => gl::Uniform1f(location, *v0),
            Double(v0) => gl::Uniform1d(location, *v0),
            Int(v0) => gl::Uniform1i(location, *v0),
            UInt(v0) => gl::Uniform1ui(location, *v0),
            FVec2(value) => gl::Uniform2fv(location, 1, value.as_ptr()),
            FVec3(value) => gl::Uniform3fv(location, 1, value.as_ptr()),
            FVec4(value) => gl::Uniform4fv(location, 1, value.as_ptr()),
            DVec2(value) => gl::Uniform2dv(location, 1, value.as_ptr()),
            DVec3(value) => gl::Uniform3dv(location, 1, value.as_ptr()),
            DVec4(value) => gl::Uniform4dv(location, 1, value.as_ptr()),
            IVec2(value) => gl::Uniform2iv(location, 1, value.as_ptr()),
            IVec3(value) => gl::Uniform3iv(location, 1, value.as_ptr()),
            IVec4(value) => gl::Uniform4iv(location, 1, value.as_ptr()),
            UIVec2(value) => gl::Uniform2uiv(location, 1, value.as_ptr()),
            UIVec3(value) => gl::Uniform3uiv(location, 1, value.as_ptr()),
            UIVec4(value) => gl::Uniform4uiv(location, 1, value.as_ptr()),
            Sampler2D(sampler) => gl::Uniform1i(location, sampler.unit as i32),
            USampler2D(sampler) => gl::Uniform1i(location, sampler.unit as i32),
        }
    }
}

pub trait UniformType: GlType + Default
where
    Self: Sized,
{
    fn get_binder(self) -> UniformValueBinder;
}

macro_rules! uniform_bind_impl {
    ($($TYPE:ty => $BINDER:ident)*) => {
         $(
         impl UniformType for $TYPE {
            fn get_binder(self) -> UniformValueBinder {
                UniformValueBinder::$BINDER(self)
            }
         }
         )*
    };
}

uniform_bind_impl!(
    f32 => Float
    f64 => Double
    i32 => Int
    u32 => UInt
    [f32; 2] => FVec2
    [f32; 3] => FVec3
    [f32; 4] => FVec4
    [f64; 2] => DVec2
    [f64; 3] => DVec3
    [f64; 4] => DVec4
    [i32; 2] => IVec2
    [i32; 3] => IVec3
    [i32; 4] => IVec4
    [u32; 2] => UIVec2
    [u32; 3] => UIVec3
    [u32; 4] => UIVec4
    Sampler2d => Sampler2D
    USampler2d => USampler2D
);
// todo matrix / images / whatever
