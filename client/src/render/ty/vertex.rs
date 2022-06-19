use glium::implement_vertex;

use crate::render::ty::mesh_builder::Quad;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PosTexVertex {
	pub position: [f32; 2],
	pub texture: [f32; 2],
}

implement_vertex!(PosTexVertex, position, texture);

impl<P: Quad<[f32; 2]>, T: Quad<[f32; 2]>> Quad<PosTexVertex> for (P, T) {
	fn expand(self) -> [PosTexVertex; 4] {
		let p = self.0.expand();
		let t = self.1.expand();
		[
			PosTexVertex {
				position: p[0],
				texture: t[0],
			},
			PosTexVertex {
				position: p[1],
				texture: t[1],
			},
			PosTexVertex {
				position: p[2],
				texture: t[2],
			},
			PosTexVertex {
				position: p[3],
				texture: t[3],
			},
		]
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PosColorVertex {
	pub position: [f32; 2],
	pub color: [f32; 4],
}

implement_vertex!(PosColorVertex, position, color);

impl<P: Quad<[f32; 2]>, C: Quad<[f32; 4]>> Quad<PosColorVertex> for (P, C) {
	fn expand(self) -> [PosColorVertex; 4] {
		let p = self.0.expand();
		let t = self.1.expand();
		[
			PosColorVertex {
				position: p[0],
				color: t[0],
			},
			PosColorVertex {
				position: p[1],
				color: t[1],
			},
			PosColorVertex {
				position: p[2],
				color: t[2],
			},
			PosColorVertex {
				position: p[3],
				color: t[3],
			},
		]
	}
}
