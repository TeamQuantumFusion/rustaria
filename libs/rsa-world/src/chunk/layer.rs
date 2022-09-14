use apollo::{FromLua, macros::*};
use rsa_core::{err::Result, log::trace};
use rsa_registry::{Id, Identifier, IdValue, Prototype, Registry, RegistryBuilder, RegPointer};

use crate::{
	AuditExt,
	chunk::block::{Block, ty::BlockPrototype},
};
use crate::chunk::block::state::BlockStateType;
use crate::chunk::block::ty::BlockType;

#[derive(Clone)]
pub struct ChunkLayerType {
	pub blocks: Registry<BlockType>,
	pub default: Id<BlockType>,
	pub collision: bool,
}

#[lua_impl]
impl ChunkLayerType {
	pub fn create_block(
		&self,
		layer_id: Id<ChunkLayerType>,
		block: RegPointer<BlockType>,
		state: Option<RegPointer<BlockStateType>>,
	) -> Result<Block> {
		let id = self.blocks.get_id(block)?;
		self.blocks[id].create(layer_id, id, state)
	}

	pub fn blocks(&self) -> &Registry<BlockType> { &self.blocks }
}

impl IdValue for ChunkLayerType {
	type Idx = u8;
}

#[derive(FromLua, Debug)]
pub struct ChunkLayerPrototype {
	pub blocks: RegistryBuilder<BlockPrototype>,
	pub default: Identifier,
	pub collision: bool,
}

impl ChunkLayerPrototype {
	pub fn bake(self) -> Result<ChunkLayerType> {
		let blocks = self.blocks.build().wrap_err("Failed to build blocks")?;
		let baked_blocks = blocks.map(|id, lookup, prototype| {
			prototype
				.bake(lookup)
				.wrap_err_with(|| format!("Failed to bake block {}", lookup.get_identifier(id)))
		})?;

		Ok(ChunkLayerType {
			default: baked_blocks
				.lookup()
				.get_id(&self.default)
				.wrap_err("Could not find default tile registered")?,
			blocks: baked_blocks,
			collision: self.collision,
		})
	}
}

impl Prototype for ChunkLayerPrototype {
	type Output = ChunkLayerType;

	fn get_name() -> &'static str { "chunk_layer" }
}
