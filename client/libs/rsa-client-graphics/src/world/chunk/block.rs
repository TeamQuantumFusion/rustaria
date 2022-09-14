use std::collections::HashSet;

use apollo::{FromLua, Lua, LuaSerdeExt, Value};
use rsa_client_core::{
	atlas::Atlas,
	debug::Debug,
	ty::{MeshBuilder, PosTexVertex},
};
use rsa_core::{
	err::Result,
	debug::DebugCategory,
	draw_debug,
	math::{size2, vec2, Rect},
	ty::{WS},
};
use rsa_core::api::util::lua_table;
use rsa_registry::{Identifier, IdValue, Prototype, RegistryBuilder, Storage};
use rsa_world::{ty::BlockPos};
use rsa_world::chunk::block::Block;
use rsa_world::chunk::block::state::{BlockStateType, BlockStates};

use crate::world::chunk;

#[derive(Debug)]
pub struct BlockRenderer {
	pub image: Rect<f32, Atlas>,
	pub states: Storage<Option<StateRenderer>, BlockStateType>,
}

impl BlockRenderer {
	pub fn mesh(
		&self,
		pos: BlockPos,
		block: &Block,
		builder: &mut MeshBuilder<PosTexVertex>,
		debug: &mut Debug,
	) {
		let mut texture = self.image;

		let variation =
			chunk::get_variation(pos) % ((texture.size.width / texture.size.height) as u32);
		let layout_width = texture.size.width / 3.0;

		let layout_height = texture.size.height;
		texture.origin.x += layout_width * variation as f32;

		if let Some(state_renderer) = &self.states[block.state] {
			texture.size.width = state_renderer.uv.size.width * layout_width;
			texture.size.height = state_renderer.uv.size.height * layout_height;
			texture.origin.x += state_renderer.uv.origin.x * layout_width;
			texture.origin.y += state_renderer.uv.origin.y * layout_height;
			let mut quad_pos = state_renderer.rect;

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
}

impl IdValue for BlockRenderer {
	type Idx = u32;
}

#[derive(Debug)]
pub struct StateRenderer {
	pub(crate) uv: Rect<f32, WS>,
	pub(crate) rect: Rect<f32, WS>,
}

impl FromLua for StateRenderer {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		let table = lua_table(lua_value)?;
		Ok(StateRenderer {
			uv: lua.from_value(table.get("uv")?)?,
			rect: lua.from_value(table.get("rect")?)?
		})
	}
}

impl IdValue for StateRenderer {
	type Idx = u16;
}

#[derive(Debug, FromLua)]
pub struct BlockRendererPrototype {
	pub image: Identifier,
	pub states: RegistryBuilder<StateRenderer>,
}

impl BlockRendererPrototype {
	pub fn bake(self, states: &BlockStates, atlas: &Atlas) -> Result<BlockRenderer> {
		Ok(BlockRenderer {
			image: atlas.get(&self.image),
			states: states.states.zip(self.states.build()?, |_, _, _, _, renderer| {
				Ok(renderer)
			})?,
		})
	}
	pub fn get_sprites(&self, sprites: &mut HashSet<Identifier>) {
		sprites.insert(self.image.clone());
	}
}

impl Prototype for BlockRendererPrototype {
	type Output = BlockRenderer;

	fn get_name() -> &'static str { "block_renderer" }
}
