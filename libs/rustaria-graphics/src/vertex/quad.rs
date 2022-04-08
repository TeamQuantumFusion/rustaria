use crate::ty::{AtlasImage, Light};
use crate::vertex::ty::{Color, ColorTextureVertex, Texture};

pub trait Quad<V> {
    fn quad(&self) -> [V; 4];
}

impl Quad<Texture> for AtlasImage<'_> {
    fn quad(&self) -> [Texture; 4] {
        let x = self.atlas.vertex_x(self.x);
        let y = self.atlas.vertex_y(self.y);
        let w = self.atlas.vertex_x(self.w);
        let h = self.atlas.vertex_y(self.h);
        [
            Texture { x, y: y + h },
            Texture { x, y },
            Texture { x: x + w, y },
            Texture { x: x + w, y: y + h },
        ]
    }
}

impl Quad<Color> for Color {
    fn quad(&self) -> [Color; 4] {
        [*self; 4]
    }
}

impl Quad<ColorTextureVertex> for (Light, AtlasImage<'_>) {
    fn quad(&self) -> [ColorTextureVertex; 4] {
        let img = self.1.quad();
        [
	        ColorTextureVertex(self.0.bl, img[0]),
	        ColorTextureVertex(self.0.tl, img[1]),
	        ColorTextureVertex(self.0.tr, img[2]),
	        ColorTextureVertex(self.0.br, img[3]),
        ]
    }
}
