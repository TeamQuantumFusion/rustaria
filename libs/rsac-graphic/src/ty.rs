use rsa_core::math::{AtlasSpace, Rect};
use crate::mesh_builder::Quad;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct PosTex {
	pos: [f32; 2],
	tex: [f32; 2],
}

glium::implement_vertex!(PosTex, pos, tex);


impl<S> Quad<PosTex> for (Rect<f32, S>, Rect<f32, AtlasSpace>) {
	fn expand(self) -> [PosTex; 4] {
		[
			PosTex {
				pos: [self.0.min_x(), self.0.min_y()],
				tex: [self.1.min_x(), self.1.min_y()],
			},
			PosTex {
				pos: [self.0.min_x(), self.0.max_y()],
				tex: [self.1.min_x(), self.1.max_y()],
			},
			PosTex {
				pos: [self.0.max_x(), self.0.max_y()],
				tex: [self.1.max_x(), self.1.max_y()],
			},
			PosTex {
				pos: [self.0.max_x(), self.0.min_y()],
				tex: [self.1.max_x(), self.1.min_y()],
			},
		]
	}
}