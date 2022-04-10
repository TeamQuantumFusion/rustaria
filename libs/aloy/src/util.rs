use opengl::gl::GLenum;

#[macro_export]
macro_rules! default {
    ($TYPE:ty => $DEF:expr) => {
        impl Default for $TYPE {
            fn default() -> Self {
                $DEF
            }
        }
    };
}

pub(crate) trait RustGlEnum {
    fn to_gl(&self) -> GLenum;
}