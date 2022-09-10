use std::collections::HashSet;

use apollo::FromLua;
use rsa_client_core::{
	atlas::Atlas,
	debug::Debug,
	ty::{MeshBuilder, PosTexVertex},
};
use rsa_core::{
	api::prototype::Prototype,
	debug::DebugCategory,
	draw_debug,
	math::{size2, vec2, Rect},
	ty::{WS},
};
use rsa_registry::Identifier;
use rsa_world::{chunk::ConnectionType, ty::BlockPos};

use crate::world::chunk;

#[derive(Debug)]
pub struct BlockRenderer {
	pub image: Rect<f32, Atlas>,
	pub connection_type: ConnectionType,
}

impl BlockRenderer {
	pub fn mesh(
		&self,
		pos: BlockPos,
		desc: &KindDesc,
		builder: &mut MeshBuilder<PosTexVertex>,
		debug: &mut Debug,
	) {
		let mut texture = self.image;

		let variation =
			chunk::get_variation(pos) % ((texture.size.width / texture.size.height) as u32);
		let layout_width = texture.size.width / 3.0;

		let layout_height = texture.size.height;
		texture.origin.x += layout_width * variation as f32;

		texture.size.width = desc.uv.size.width * layout_width;
		texture.size.height = desc.uv.size.height * layout_height;
		texture.origin.x += desc.uv.origin.x * layout_width;
		texture.origin.y += desc.uv.origin.y * layout_height;
		let mut quad_pos = desc.rect;

		quad_pos.origin += size2(pos.x() as f32, pos.y() as f32);

		const VARIATION_COLORS: [u32; 3] = [0xff0000, 0x00ff00, 0x0000ff];

		draw_debug!(
			debug,
			DebugCategory::ChunkMeshing,
			vec2(pos.x() as f32 + 0.5, pos.y() as f32 + 0.5),
			VARIATION_COLORS[(variation % 3) as usize],
			5.0,
			0.5
		);
		builder.push_quad((quad_pos, texture));
	}
}

pub struct KindDesc {
	pub(crate) uv: Rect<f32, WS>,
	pub(crate) rect: Rect<f32, WS>,
}

#[derive(Debug, FromLua)]
pub struct BlockRendererPrototype {
	pub image: Identifier,
	pub connection_type: ConnectionType,
}

impl BlockRendererPrototype {
	pub fn bake(&self, atlas: &Atlas) -> BlockRenderer {
		BlockRenderer {
			image: atlas.get(&self.image),
			connection_type: self.connection_type,
		}
	}
	pub fn get_sprites(&self, sprites: &mut HashSet<Identifier>) {
		sprites.insert(self.image.clone());
	}
}

impl Prototype for BlockRendererPrototype {
	type Output = BlockRenderer;

	fn get_name() -> &'static str { "block_renderer" }
}
