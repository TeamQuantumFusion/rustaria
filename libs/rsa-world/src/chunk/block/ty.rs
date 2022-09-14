use apollo::FromLua;
use apollo::macros::lua_impl;
use rsa_registry::{Id, IdValue, Prototype, RegistryLookup, RegPointer};
use crate::{AuditExt, BlockStateType, ChunkLayerType};
use crate::chunk::block::Block;
use crate::chunk::block::state::{BlockStates, BlockStatesPrototype};
use crate::spread::block::{BlockSpreader, BlockSpreaderPrototype};

#[derive(FromLua, Debug)]
pub struct BlockPrototype {
	pub collision: bool,
	pub states: BlockStatesPrototype,
	pub spread: Option<BlockSpreaderPrototype>,
}

#[lua_impl]
impl BlockPrototype {
	pub fn bake(self, blocks: &RegistryLookup<BlockType>) -> crate::Result<BlockType> {
		Ok(BlockType {
			collision: self.collision,
			states: self.states.bake()?,
			spread: if let Some(spread) = self.spread {
				Some(spread.bake(blocks).wrap_err("Could not bake spreader")?)
			} else {
				None
			},
		})
	}
}

impl Prototype for BlockPrototype {
	type Output = BlockType;

	fn get_name() -> &'static str { "block" }
}

#[derive(Clone)]
pub struct BlockType {
	pub collision: bool,
	pub states: BlockStates,
	pub spread: Option<BlockSpreader>,
}

impl BlockType {
	pub fn create(&self, layer_id: Id<ChunkLayerType>, block_id: Id<BlockType>, state: Option<RegPointer<BlockStateType>>) -> crate::Result<Block> {
		Ok(Block {
			id: block_id,
			layer: layer_id,
			state: self.states.get_block_state(state)?,
			collision: self.collision,
		})
	}
}

impl IdValue for BlockType {
	type Idx = u32;
}
