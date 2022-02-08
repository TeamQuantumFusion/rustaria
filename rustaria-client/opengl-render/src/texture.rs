use std::ffi::c_void;

use opengl::gl;
use opengl::gl::types::GLenum;

use crate::raw::RawTexture;
use crate::types::GlType;
use crate::uniform::{UniformType, UniformValueBinder};
use crate::util::RustGlEnum;

pub struct Texture {
    raw: RawTexture,
    target: GLenum,
}

impl Texture {
    pub fn new<T: GlType>(
        ty: TextureType<T>,
        desc: TextureDescriptor,
    ) -> Texture {
        let target = ty.to_gl();

        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
            desc.apply(target);

            ty.upload();

            Texture {
                raw: RawTexture { gl_id: id },
                target
            }
        }
    }

    pub(crate) unsafe fn bind(&self) {
        gl::BindTexture(self.target, self.raw.gl_id);
    }
}

default!(TextureDescriptor => TextureDescriptor {
    wrap: Default::default(),
    lod: Default::default(),
    min_filter: Default::default(),
    mag_filter: Default::default()
});
pub struct TextureDescriptor {
    pub wrap: TextureWrap,
    pub lod: TextureLod,
    pub min_filter: TextureMinFilter,
    pub mag_filter: TextureMagFilter,
}

impl TextureDescriptor {
    unsafe fn apply(&self, target: GLenum) {
        self.lod.apply(target);
        self.wrap.apply(target);
        gl::TexParameteri(
            target,
            gl::TEXTURE_MIN_FILTER,
            self.min_filter.to_gl() as i32,
        );
        gl::TexParameteri(
            target,
            gl::TEXTURE_MAG_FILTER,
            self.mag_filter.to_gl() as i32,
        );
    }
}

// Texture Type
pub enum TextureType<T: GlType> {
    Texture2d {
        images: Option<Vec<TextureData<T>>>,
        internal: InternalFormat,
        width: u32,
        height: u32,
        border: i32,
    },
}

impl<T: GlType> TextureType<T> {
    unsafe fn upload(&self) {
        match self {
            TextureType::Texture2d {
                images,
                internal,
                width,
                height,
                border,
            } => {
                if let Some(images) = images {
                    for (level, data) in images.iter().enumerate() {
                        gl::TexImage2D(
                            self.to_gl(),
                            level as i32,
                            internal.to_gl() as i32,
                            *width as i32,
                            *height as i32,
                            *border,
                            data.texture_format.to_gl(),
                            T::gl_enum(),
                            data.texture_data.as_ptr() as *const c_void,
                        );
                    }
                }
            }
        }
    }
}

impl<T: GlType> RustGlEnum for TextureType<T> {
    fn to_gl(&self) -> GLenum {
        match self {
            TextureType::Texture2d { .. } => gl::TEXTURE_2D,
        }
    }
}

// Internal Format
pub enum InternalFormat {
    DepthComponent,
    DepthStencil,
    Red,
    Rg,
    Rgb,
    Rgba,
    // add sized formats later. lol
}

impl RustGlEnum for InternalFormat {
    fn to_gl(&self) -> GLenum {
        match self {
            InternalFormat::DepthComponent => gl::DEPTH_COMPONENT,
            InternalFormat::DepthStencil => gl::DEPTH_STENCIL,
            InternalFormat::Red => gl::RED,
            InternalFormat::Rg => gl::RG,
            InternalFormat::Rgb => gl::RGB,
            InternalFormat::Rgba => gl::RGBA,
        }
    }
}

// Texture Data
pub struct TextureData<T: GlType> {
    pub texture_data: Vec<T>,
    pub texture_format: TextureDataFormat,
}

pub enum TextureDataFormat {
    Red,
    Rg,
    Rgb,
    Rgba,
    Bgr,
    Bgra,
}

impl RustGlEnum for TextureDataFormat {
    fn to_gl(&self) -> GLenum {
        match self {
            TextureDataFormat::Red => gl::RED,
            TextureDataFormat::Rg => gl::RG,
            TextureDataFormat::Rgb => gl::RGB,
            TextureDataFormat::Rgba => gl::RGBA,
            TextureDataFormat::Bgr => gl::BGR,
            TextureDataFormat::Bgra => gl::BGRA,
        }
    }
}

// Lod
default!(TextureLod => TextureLod {
    max_level: 1000,
    min: -1000.0,
    max: 1000.0
});
pub struct TextureLod {
    pub max_level: i32,
    pub min: f32,
    pub max: f32,
}

impl TextureLod {
    unsafe fn apply(&self, target: GLenum) {
        gl::TexParameteri(target, gl::TEXTURE_MAX_LEVEL, self.max_level);
        gl::TexParameterf(target, gl::TEXTURE_MIN_LOD, self.min);
        gl::TexParameterf(target, gl::TEXTURE_MAX_LOD, self.max);
    }
}

// Wrap
default!(TextureWrap => TextureWrap {
    x: Default::default(),
    y: Default::default(),
    z: Default::default(),
});
pub struct TextureWrap {
    pub x: WrapType,
    pub y: WrapType,
    pub z: WrapType,
}

impl TextureWrap {
    unsafe fn apply(&self, target: GLenum) {
        gl::TexParameteri(target, gl::TEXTURE_WRAP_S, self.x.to_gl() as i32);
        gl::TexParameteri(target, gl::TEXTURE_WRAP_T, self.y.to_gl() as i32);
        gl::TexParameteri(target, gl::TEXTURE_WRAP_R, self.z.to_gl() as i32);
    }
}

default!(WrapType => WrapType::Repeat);
pub enum WrapType {
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Repeat,
    MirrorClampToEdge,
}

impl RustGlEnum for WrapType {
    fn to_gl(&self) -> GLenum {
        match self {
            WrapType::ClampToEdge => gl::CLAMP_TO_EDGE,
            WrapType::ClampToBorder => gl::CLAMP_TO_BORDER,
            WrapType::MirroredRepeat => gl::MIRRORED_REPEAT,
            WrapType::Repeat => gl::REPEAT,
            WrapType::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE,
        }
    }
}

// Filter
default!(TextureMinFilter =>
TextureMinFilter::Mipmap(
    FilterType::Nearest,
    FilterType::Linear
));
pub enum TextureMinFilter {
    Basic(FilterType),
    Mipmap(FilterType, FilterType),
}

impl RustGlEnum for TextureMinFilter {
    fn to_gl(&self) -> GLenum {
        match self {
            TextureMinFilter::Basic(ty) => ty.to_gl(),
            TextureMinFilter::Mipmap(from, to) => match from {
                FilterType::Nearest => match to {
                    FilterType::Nearest => gl::NEAREST_MIPMAP_NEAREST,
                    FilterType::Linear => gl::NEAREST_MIPMAP_LINEAR,
                },
                FilterType::Linear => match to {
                    FilterType::Nearest => gl::LINEAR_MIPMAP_NEAREST,
                    FilterType::Linear => gl::LINEAR_MIPMAP_LINEAR,
                },
            },
        }
    }
}

default!(TextureMagFilter => TextureMagFilter(FilterType::Linear));
pub struct TextureMagFilter(pub FilterType);

impl RustGlEnum for TextureMagFilter {
    fn to_gl(&self) -> GLenum {
        self.0.to_gl()
    }
}

pub enum FilterType {
    Nearest,
    Linear,
}

impl RustGlEnum for FilterType {
    fn to_gl(&self) -> GLenum {
        match self {
            FilterType::Nearest => gl::NEAREST,
            FilterType::Linear => gl::LINEAR,
        }
    }
}

// Sampelr stuff
#[derive(Copy, Clone)]
pub struct Sampler2d {
    pub(crate) unit: u8
}

impl GlType for Sampler2d {
    fn gl_enum() -> GLenum {
        gl::SAMPLER_2D
    }
}

impl Default for Sampler2d {
    fn default() -> Self {
        Self  {
            unit: 0
        }
    }
}