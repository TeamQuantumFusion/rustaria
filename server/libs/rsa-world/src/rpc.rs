use apollo::Lua;
use rsa_core::{api::stargate::Stargate, err::Result};
use rsa_hash::Hasher;
use rsa_registry::Registry;

use crate::{
	chunk::layer::BlockLayerPrototype, entity::prototype::EntityPrototype, BlockLayer, EntityDesc,
};

#[derive(Default)]
pub struct WorldRPC {
	pub block_layer: Registry<BlockLayer>,
	pub entity: Registry<EntityDesc>,
}

impl WorldRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<BlockLayerPrototype>(lua)?;
		stargate.register_builder::<EntityPrototype>(lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<WorldRPC> {
		Ok(WorldRPC {
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
