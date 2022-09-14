pub mod state;
pub mod ty;

use apollo::{macros::*};

use rsa_registry::{Id, IdValue, RegPointer};

use ty::BlockType;

use crate::{AuditExt, ChunkLayerType, spread::block::BlockSpreader};
use crate::chunk::block::state::{BlockStates, BlockStateType};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Block  {
	pub id: Id<BlockType>,
	pub layer: Id<ChunkLayerType>,
	pub state: Id<BlockStateType>,
	pub collision: bool,
}

#[lua_impl]
impl Block {
	#[lua_method]
	pub fn get_collision(&self) -> bool {
		self.collision
	}
}