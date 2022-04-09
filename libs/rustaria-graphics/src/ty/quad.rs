use aloy::atlas::AtlasLocation;
use crate::ty::{Color, Pos, Rectangle, Texture};

pub trait Quad {
    type Item: Copy;
    fn quad(&self) -> [Self::Item; 4];
}

impl<K: Quad, V: Quad> Quad for (K, V) {
    type Item = (K::Item, V::Item);

    fn quad(&self) -> [Self::Item; 4] {
        let k = self.0.quad();
        let v = self.1.quad();
        [
            (k[0], v[0]),
            (k[1], v[1]),
            (k[2], v[2]),
            (k[3], v[3]),
        ]
    }
}

impl<V0: Quad, V1: Quad, V2: Quad> Quad for (V0, V1, V2) {
    type Item = (V0::Item, V1::Item, V2::Item);

    fn quad(&self) -> [Self::Item; 4] {
        let v0 = self.0.quad();
        let v1 = self.1.quad();
        let v2 = self.2.quad();
        [
            (v0[0], v1[0], v2[0]),
            (v0[1], v1[1], v2[1]),
            (v0[2], v1[2], v2[2]),
            (v0[3], v1[3], v2[3]),
        ]
    }
}

impl Quad for AtlasLocation {
    type Item = Texture;

    fn quad(&self) -> [Texture; 4] {
        let x = self.x;
        let y = self.y;
        let w = self.width;
        let h = self.height;
        [
            Texture { x, y: y + h },
            Texture { x, y },
            Texture { x: x + w, y },
            Texture { x: x + w, y: y + h },
        ]
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
