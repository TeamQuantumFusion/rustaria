use eyre::{ContextCompat, WrapErr};
use apollo::prelude::LuaResult;
use tracing::error_span;

use crate::{
	api::{
		luna::{lib::registry_builder::RegistryBuilder, table::LunaTable},
		prototype::Prototype,
		registry::Registry,
	},
	ty::{id::Id, identifier::Identifier},
	util::blake3::Hasher,
	world::chunk::block::{BlockDesc, BlockPrototype},
};
use apollo::impl_macro::*;

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

pub struct BlockLayerPrototype {
	pub blocks: Registry<BlockPrototype>,
	pub default: Identifier,
	pub collision: bool,
}

impl BlockLayerPrototype {
	pub fn bake(self) -> eyre::Result<BlockLayer> {
		let lookup = self
			.blocks
			.ident_to_id
			.iter()
			.map(|(ident, id)| (ident.clone(), id.build()))
			.collect();

		let mut out = Vec::new();
		for (id, ident, entry) in self.blocks.into_entries() {
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

	fn from_lua(table: LunaTable) -> eyre::Result<Self> {
		let mut blocks = RegistryBuilder::<BlockPrototype>::new();
		blocks.register(table.lua, table.get("blocks")?)?;
		Ok(BlockLayerPrototype {
			blocks: blocks
				.build(table.lua)
				.wrap_err("Failed to create blocks registry")?,
			default: table.get("default")?,
			collision: table.get("collision")?,
		})
	}
}
