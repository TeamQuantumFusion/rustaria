pub mod quad;


pub struct Light {
    pub bl: Color,
    pub tl: Color,
    pub tr: Color,
    pub br: Color,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Texture {
    pub x: f32,
    pub y: f32,
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color {
            r: value[0] as f32 / 255.0,
            g: value[1] as f32 / 255.0,
            b: value[2] as f32 / 255.0,
        }
    }
}


pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}


pub struct Player {
    pub pos: [f32; 2],
    pub zoom: f32
}