use anyways::ext::AuditExt;
use anyways::Result;
use apollo::prelude::LuaResult;
use tracing::{error_span, trace};
use apollo::{FromLua, Lua, Value};

use crate::{
	api::{
		luna::{lib::registry_builder::RegistryBuilder, table::LunaTable},
		prototype::Prototype,
		registry::Registry,
	},
	ty::{id::Id, identifier::Identifier},
	world::chunk::block::{BlockDesc, BlockPrototype},
};
use apollo::macros::*;

#[derive(Clone)]
pub struct BlockLayer {
	pub blocks: Registry<BlockDesc>,
	pub default: Id<BlockDesc>,
	pub collision: bool,
}

#[lua_impl]
impl BlockLayer {
	#[lua_field(get blocks)]
	pub fn blocks(&self) -> &Registry<BlockDesc> {
		&self.blocks
	}
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
		let lookup = blocks
			.ident_to_id
			.iter()
			.map(|(ident, id)| (ident.clone(), id.build()))
			.collect();

		let mut out = Vec::new();
		for (id, ident, entry) in blocks.into_entries() {
			trace!("Baked block {ident} {entry:?}");
			let prototype = entry
				.bake(&lookup)
				.wrap_err_with(|| format!("Failed to bake block {}", ident))?;
			out.push((id.build(), ident, prototype));
		}

		let registry: Registry<BlockDesc> = out.into_iter().collect();
		Ok(BlockLayer {
			default: *registry
				.ident_to_id
				.get(&self.default)
				.wrap_err("Could not find default tile registered")?,
			blocks: registry,
			collision: self.collision,
		})
	}
}


impl Prototype for BlockLayerPrototype {
	type Output = BlockLayer;

	fn get_name() -> &'static str { "block_layer" }
}