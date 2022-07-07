use rsa_core::api::prototype::Prototype;
use rsa_core::log::trace;
use rsa_core::err::Result;
use rsa_core::ty::{Id, Identifier, Registry, RegistryBuilder};
use crate::{AuditExt, BlockDesc};
use crate::chunk::block::BlockPrototype;
use apollo::macros::*;
use apollo::FromLua;

#[derive(Clone)]
pub struct BlockLayer {
	pub blocks: Registry<BlockDesc>,
	pub default: Id<BlockDesc>,
	pub collision: bool,
}

#[lua_impl]
impl BlockLayer {
	#[lua_field(get blocks)]
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
