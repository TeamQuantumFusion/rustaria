use apollo::{macros::*, Lua};
use rsa_core::{
	api::stargate::Stargate,
	err::{ext::AuditExt, Result},
};
use rsa_hash::Hasher;
use rsa_registry::{Identifier, Registry};

use crate::{
	chunk::{block::Block, layer::ChunkLayerPrototype},
	entity::prototype::EntityPrototype,
	ChunkLayerType, EntityType,
};

#[derive(Default)]
pub struct WorldAPI {
	pub block_layer: Registry<ChunkLayerType>,
	pub entity: Registry<EntityType>,
}

#[lua_impl]
impl WorldAPI {
	#[lua_method]
	pub fn create_block(
		&self,
		layer: Identifier,
		block: Identifier,
		state: Option<Identifier>,
	) -> Result<Block> {
		let layer_id = self
			.block_layer
			.lookup()
			.get_id(&layer)
			.wrap_err_with(|| format!("Could not find BlockLayer \"{layer}\""))?;

		let block_layer = &self.block_layer[layer_id];

		let state = state.as_ref();
		block_layer
			.create_block(layer_id, (&block).into(), state.map(|v| v.into()))
			.wrap_err("Failed to create block")
	}

	#[lua_field(get blockLayer)]
	pub fn block_layer(&self) -> &Registry<ChunkLayerType> { &self.block_layer }

	#[lua_field(get entity)]
	pub fn entity(&self) -> &Registry<EntityType> { &self.entity }

	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<ChunkLayerPrototype>(lua)?;
		stargate.register_builder::<EntityPrototype>(lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<WorldAPI> {
		Ok(WorldAPI {
			block_layer: stargate
				.build_registry::<ChunkLayerPrototype>()?
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
