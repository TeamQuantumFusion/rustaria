use std::collections::HashMap;

use rustaria::api::rendering::{Pane, RenderingSystem};
use rustaria_util::math::rect;
use rustariac_backend::{ClientBackend, ty::AtlasLocation};
use rustariac_backend::builder::VertexBuilder;
use rustariac_backend::ty::{Camera, PosTexture};

pub mod chunk_drawer;
pub mod entity_drawer;

pub enum BakedRenderingSystem {
	Static(BakedPane),
	State(HashMap<String, BakedPane>),
}

impl BakedRenderingSystem {
	pub fn new(system: &RenderingSystem, backend: &ClientBackend) -> BakedRenderingSystem {
		match system {
			RenderingSystem::Static(pane) => {
				BakedRenderingSystem::Static(BakedPane::new(pane, backend))
			}
			RenderingSystem::State(states) => BakedRenderingSystem::State(
				states
					.iter()
					.map(|(state, pane)| (state.clone(), BakedPane::new(pane, backend)))
					.collect(),
			),
		}
	}

	pub fn push(&self, builder: &mut VertexBuilder<PosTexture>, camera: &Camera, x: f32, y: f32) {
		match self {
			BakedRenderingSystem::Static(pane) => pane.push(builder, camera, x, y),
			BakedRenderingSystem::State(_) => todo!(),
		}
	}
}

pub struct BakedPane {
	pub x_offset: f32,
	pub y_offset: f32,
	pub width: f32,
	pub height: f32,
	pub sprite: AtlasLocation,
}

impl BakedPane {
	pub fn new(pane: &Pane, backend: &ClientBackend) -> BakedPane {
		let instance = backend.instance();
		BakedPane {
			x_offset: pane.x_offset,
			y_offset: pane.y_offset,
			width: pane.width,
			height: pane.height,
			sprite: instance.atlas.get(&pane.sprite),
		}
	}

	pub fn push(&self, builder: &mut VertexBuilder<PosTexture>, camera: &Camera, x: f32, y: f32) {
		let rectangle = rect(
			x + self.x_offset,
			y + self.y_offset,
			self.width,
			self.height,
		);
		if camera.visible().intersects(&rectangle) {
			builder.quad((rectangle, self.sprite))
		}
	}
}
