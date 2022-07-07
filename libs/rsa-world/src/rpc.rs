use apollo::Lua;
use rsa_core::{
	api::{
		stargate::Stargate,
	},
	blake3::Hasher,
	err::Result,
	ty::Registry,
};

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
		let registry = stargate.build_registry::<BlockLayerPrototype>()?;

		let block_layer = registry
			.table
			.into_iter()
			.zip(registry.id_to_ident.into_iter());

		let mut out = Vec::new();
		for ((id, prototype), (_, identifier)) in block_layer {
			out.push((id.build(), identifier, prototype.bake()?));
		}

		Ok(WorldRPC {
			block_layer: out.into_iter().collect(),
			entity: stargate
				.build_registry::<EntityPrototype>()?
				.into_entries()
				.map(|(id, ident, prototype)| (id.build(), ident, prototype.bake(id)))
				.collect(),
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.block_layer.append_hasher(hasher);
		self.entity.append_hasher(hasher);
	}
}
