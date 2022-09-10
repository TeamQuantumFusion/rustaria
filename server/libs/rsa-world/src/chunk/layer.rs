use apollo::{FromLua, macros::*};
use rsa_core::{
	api::prototype::Prototype,
	err::Result,
	log::trace,
};
use rsa_registry::{Id, Identifier, Registry, RegistryBuilder};

use crate::{AuditExt, BlockDesc};
use crate::chunk::block::prototype::BlockPrototype;

#[derive(Clone)]
pub struct BlockLayer {
	pub blocks: Registry<BlockDesc>,
	pub default: Id<BlockDesc>,
	pub collision: bool,
}

#[lua_impl]
impl BlockLayer {
	pub fn blocks(&self) -> &Registry<BlockDesc> { &self.blocks }
}

#[derive(FromLua, Debug)]
pub struct BlockLayerPrototype {
	pub blocks: RegistryBuilder<BlockPrototype>,
	pub default: Identifier,
	pub collision: bool,
}

impl BlockLayerPrototype {
	pub fn bake(self) -> Result<BlockLayer> {
		let blocks = self.blocks.build().wrap_err("Failed to build blocks")?;
		let baked_blocks = blocks.map(|id, lookup, prototype| {
			prototype.bake(lookup).wrap_err_with(|| format!("Failed to bake block {}", lookup.get_identifier(id)))
		})?;

		Ok(BlockLayer {
			default: baked_blocks.lookup().get_id(&self.default)
				.wrap_err("Could not find default tile registered")?,
			blocks: baked_blocks,
			collision: self.collision,
		})
	}
}

impl Prototype for BlockLayerPrototype {
	type Output = BlockLayer;

	fn get_name() -> &'static str { "block_layer" }
}
