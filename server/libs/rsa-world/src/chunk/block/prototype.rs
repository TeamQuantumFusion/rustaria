use apollo::FromLua;
use apollo::macros::lua_impl;
use rsa_core::api::prototype::Prototype;
use std::collections::HashMap;
use rsa_registry::{Id, Identifier, RegistryLookup};
use crate::{AuditExt, BlockDesc};
use crate::chunk::block::state::BlockStates;
use crate::spread::block::BlockSpreaderPrototype;

#[derive(FromLua, Debug)]
pub struct BlockPrototype {
	pub collision: bool,
	pub spread: Option<BlockSpreaderPrototype>,
}

#[lua_impl]
impl BlockPrototype {
	pub fn bake(self, blocks: &RegistryLookup<BlockDesc>) -> crate::Result<BlockDesc> {
		Ok(BlockDesc {
			collision: self.collision,
			spread: if let Some(spread) = self.spread {
				Some(spread.bake(blocks).wrap_err("Could not bake spreader")?)
			} else {
				None
			},
		})
	}
}

impl Prototype for BlockPrototype {
	type Output = BlockDesc;

	fn get_name() -> &'static str { "block" }
}
