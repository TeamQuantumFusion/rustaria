use std::ops::Add;

use opengl::gl::types::GLenum;

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

pub struct IdTable {
    empty_data: Vec<usize>,
    current: usize,
}

impl IdTable {
    pub fn join(&mut self) -> usize {
        if self.empty_data.is_empty() {
            let pos = self.current;
            self.current += 1;
            pos
        } else {
            self.empty_data.pop().unwrap()
        }
    }

    pub fn leave(&mut self, pos: usize) {
        self.empty_data.push(pos);
    }
}