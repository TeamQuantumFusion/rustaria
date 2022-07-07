use std::collections::HashSet;

use apollo::{FromLua, Function, Lua, LuaSerdeExt};
use rsa_client_core::{
	atlas::Atlas,
	debug::Debug,
	ty::{MeshBuilder, PosTexVertex},
};
use rsa_core::{
	api::prototype::Prototype,
	err::{ext::AuditExt, Result},
	ty::{DirMap, Direction, IdTable, Identifier, RegistryBuilder},
};
use rsa_world::{
	chunk::{
		block::{Block, BlockDesc},
		layer::BlockLayer,
		ChunkLayer, ConnectionType,
	},
	ty::{BlockPos, ChunkPos},
};

use crate::world::{
	chunk::block::{BlockRenderer, BlockRendererPrototype, KindDesc},
	neighbor::{NeighborMatrixBuilder, SpriteConnectionKind},
};

pub struct BlockLayerRenderer {
	block_renderers: IdTable<BlockDesc, Option<BlockRenderer>>,
	kind_descs: Vec<KindDesc>,
}

impl BlockLayerRenderer {
	pub fn mesh_chunk_layer(
		&self,
		chunk: ChunkPos,
		layer: &ChunkLayer<Block>,
		neighbors: DirMap<Option<&ChunkLayer<Block>>>,
		builder: &mut MeshBuilder<PosTexVertex>,
		debug: &mut Debug,
	) {
		let func = |tile: &Block| {
			self.block_renderers
				.get(tile.id)
				.as_ref()
				.map(|renderer| renderer.connection_type)
		};

		let mut matrix = NeighborMatrixBuilder::new(layer.map(ConnectionType::Isolated, func));
		matrix.compile_internal();

		for dir in Direction::values() {
			if let Some(neighbor) = neighbors[dir] {
				matrix.compile_edge(dir, &neighbor.map(ConnectionType::Isolated, func));
			}
		}

		let connection_layer = matrix.export();
		layer.entries(|entry, connection| {
			if let Some(renderer) = self.block_renderers.get(connection.id) {
				renderer.mesh(
					BlockPos::new(chunk, entry),
					&self.kind_descs[connection_layer[entry] as u8 as usize],
					builder,
					debug,
				);
			}
		});
	}
}

#[derive(Debug, FromLua)]
pub struct BlockLayerRendererPrototype {
	pub blocks: RegistryBuilder<BlockRendererPrototype>,
	pub get_uv: Function,
	pub get_rect: Function,
}

impl BlockLayerRendererPrototype {
	pub fn bake(self, lua: &Lua, atlas: &Atlas, parent: &BlockLayer) -> Result<BlockLayerRenderer> {
		let mut kind_uvs = Vec::new();
		for value in SpriteConnectionKind::iter() {
			let value = format!("{:?}", value);
			kind_uvs.push(KindDesc {
				uv: lua
					.from_value(
						self.get_uv
							.call(value.clone())
							.wrap_err("Failed to get uv.")?,
					)
					.wrap_err("Failed to get uv result.")?,
				rect: lua
					.from_value(self.get_rect.call(value).wrap_err("Failed to get rect.")?)
					.wrap_err("Failed to get rect result.")?,
			});
		}

		let blocks = self.blocks.build()?;
		Ok(BlockLayerRenderer {
			block_renderers: parent
				.blocks
				.id_to_ident
				.iter()
				.map(|(id, entry)| {
					(
						id,
						blocks
							.ident_to_id
							.get(entry)
							.map(|entry| blocks.get(*entry).bake(atlas)),
					)
				})
				.collect(),
			kind_descs: kind_uvs,
		})
	}

	pub fn get_sprites(&self, sprites: &mut HashSet<Identifier>) {
		for (_, entry) in self.blocks.values.values() {
			entry.get_sprites(sprites);
		}
	}
}

impl Prototype for BlockLayerRendererPrototype {
	type Output = BlockLayerRenderer;

	fn get_name() -> &'static str { "block_layer_renderer" }
}
