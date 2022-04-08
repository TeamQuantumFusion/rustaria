#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Texture {
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorTextureVertex(pub Color, pub Texture);


impl From<[u8; 3]> for Color {
	fn from(value: [u8; 3]) -> Self {
		Color {
			r: value[0] as f32 / 255.0,
			g: value[1] as f32 / 255.0,
			b: value[2] as f32 / 255.0
		}
	}
}

impl From<(Color, Texture)> for ColorTextureVertex {
	fn from(value: (Color, Texture)) -> Self {
		ColorTextureVertex(value.0, value.1)
	}
}