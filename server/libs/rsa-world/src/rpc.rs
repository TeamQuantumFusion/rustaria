use apollo::{macros::*, Lua};
use rsa_core::{
	api::stargate::Stargate,
	err::{ext::AuditExt, Result},
};
use rsa_hash::Hasher;
use rsa_registry::{Identifier, Registry};

use crate::{
	chunk::{block::Block, layer::BlockLayerPrototype},
	entity::prototype::EntityPrototype,
	BlockLayer, EntityDesc,
};

#[derive(Default)]
pub struct WorldAPI {
	pub block_layer: Registry<BlockLayer>,
	pub entity: Registry<EntityDesc>,
}

#[lua_impl]
impl WorldAPI {
	#[lua_method]
	pub fn create_block(&self, layer: Identifier, block: Identifier) -> Result<Block> {
		let layer_id = self
			.block_layer
			.lookup()
			.get_id(&layer)
			.wrap_err_with(|| format!("Could not find BlockLayer \"{layer}\""))?;

		let block_layer = &self.block_layer[layer_id];

		let id = block_layer
			.blocks
			.lookup()
			.get_id(&block)
			.wrap_err_with(|| format!("Could not find Block \"{block}\" on layer \"{layer}\""))?;
		let desc = &block_layer.blocks[id];
		let block = desc.create(id, layer_id);
		Ok(block)
	}

	#[lua_field(get blockLayer)]
	pub fn block_layer(&self) -> &Registry<BlockLayer> { &self.block_layer }

	#[lua_field(get entity)]
	pub fn entity(&self) -> &Registry<EntityDesc> { &self.entity }

	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<BlockLayerPrototype>(lua)?;
		stargate.register_builder::<EntityPrototype>(lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<WorldAPI> {
		Ok(WorldAPI {
			block_layer: stargate
				.build_registry::<BlockLayerPrototype>()?
				.map(|_, _, prototype| prototype.bake())?,
			entity: stargate
				.build_registry::<EntityPrototype>()?
				.map(|id, _, prototype| Ok(prototype.bake(id)))?,
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.block_layer.lookup().append_hasher(hasher);
		self.entity.lookup().append_hasher(hasher);
	}
}