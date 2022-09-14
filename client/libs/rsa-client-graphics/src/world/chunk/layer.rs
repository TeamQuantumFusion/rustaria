use std::collections::HashSet;

use apollo::{FromLua, Function, Lua, LuaSerdeExt};
use rsa_client_core::{
	atlas::Atlas,
	debug::Debug,
	ty::{MeshBuilder, PosTexVertex},
};
use rsa_core::{
	err::{ext::AuditExt, Result},
	ty::{Direction, DirMap},
};
use rsa_registry::{Identifier, IdValue, Prototype, Registry, RegistryBuilder, Storage};
use rsa_world::{
	chunk::{
		block::{Block, state::BlockStateType},
		ChunkLayer,
		layer::ChunkLayerType,
	},
	ty::{BlockPos, ChunkPos},
};
use rsa_world::chunk::block::ty::BlockType;

use crate::world::chunk::block::{BlockRenderer, BlockRendererPrototype, StateRenderer};

pub struct ChunkLayerRenderer {
	block_renderers: Storage<Option<BlockRenderer>, BlockType>,
}

impl ChunkLayerRenderer {
	pub fn mesh_chunk_layer(
		&self,
		chunk: ChunkPos,
		layer: &ChunkLayer,
		builder: &mut MeshBuilder<PosTexVertex>,
		debug: &mut Debug,
	) {
		layer.entries(|entry, block| {
			if let Some(renderer) = &self.block_renderers[block.id] {
				renderer.mesh(
					BlockPos::new(chunk, entry),
					block,
					builder,
					debug,
				);
			}
		});
	}
}

impl IdValue for ChunkLayerRenderer {
	type Idx = u16;
}

#[derive(Debug, FromLua)]
pub struct ChunkLayerRendererPrototype {
	pub blocks: RegistryBuilder<BlockRendererPrototype>,
}

impl ChunkLayerRendererPrototype {
	pub fn bake(self, lua: &Lua, atlas: &Atlas, parent: &ChunkLayerType) -> Result<ChunkLayerRenderer> {
		Ok(ChunkLayerRenderer {
			block_renderers: parent
				.blocks
				.zip(self.blocks.build()?, |id, idd, lookup, desc, prototype| {
					prototype.bake(&desc.states, atlas)
				})?,
		})
	}

	pub fn get_sprites(&self, sprites: &mut HashSet<Identifier>) {
		for (_, (_, entry)) in self.blocks.values.iter() {
			entry.get_sprites(sprites);
		}
	}
}

impl Prototype for ChunkLayerRendererPrototype {
	type Output = ChunkLayerRenderer;

	fn get_name() -> &'static str { "block_layer_renderer" }
}
