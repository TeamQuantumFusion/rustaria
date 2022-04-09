use aloy::atlas::AtlasLocation;
use aloy::vertex::Quad;

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

pub type Texture = Pos;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
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


impl From<AtlasLocation> for Rectangle {
    fn from(value: AtlasLocation) -> Self {
        Rectangle  {
            x: value.x,
            y: value.y,
            w: value.width,
            h: value.height
        }
    }
}

impl From<(f32, f32)> for Texture {
    fn from(value: (f32, f32)) -> Self {
        Texture {
            x: value.0,
            y: value.1
        }
    }
}


impl Quad for Rectangle {
    type Item = Pos;

    fn quad(&self) -> [Pos; 4] {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;
        [
            Pos { x, y: y + h },
            Pos { x, y },
            Pos { x: x + w, y },
            Pos { x: x + w, y: y + h },
        ]
    }
}

impl Quad for Color {
    type Item = Color;

    fn quad(&self) -> [Color; 4] {
        [*self; 4]
    }
}

pub struct Player {
    pub pos: [f32; 2],
    pub zoom: f32
}
