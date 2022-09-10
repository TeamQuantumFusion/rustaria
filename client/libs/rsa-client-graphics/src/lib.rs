#![feature(drain_filter)]

use std::{collections::HashSet, mem::replace};

use apollo::{macros::*, Lua};
use rsa_client_core::{atlas::Atlas, frontend::Frontend};
use rsa_core::{
	api::{stargate::Stargate, Core},
	err::{ext::AuditExt, Result},
};
use rsa_registry::Storage;
use rsa_world::{chunk::layer::BlockLayer, entity::prototype::EntityDesc};
use rustaria_server::api::RustariaAPI;

use crate::world::{
	chunk::layer::{BlockLayerRenderer, BlockLayerRendererPrototype},
	entity::{EntityRenderer, EntityRendererPrototype},
};

pub mod world;

#[derive(Default)]
pub struct GraphicsRPC {
	pub block_layer_renderer: Storage<Option<BlockLayerRenderer>, BlockLayer>,
	pub entity_renderer: Storage<Option<EntityRenderer>, EntityDesc>,
	pub atlas: Option<Atlas>,
}

impl GraphicsRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<BlockLayerRendererPrototype>(lua)?;
		stargate.register_builder::<EntityRendererPrototype>(lua)?;
		Ok(())
	}

	pub fn build(
		frontend: &Frontend,
		server: &RustariaAPI,
		core: &Core,
		stargate: &mut Stargate,
	) -> Result<GraphicsRPC> {
		let mut sprites = HashSet::new();
		let block_layers = stargate.build_registry::<BlockLayerRendererPrototype>()?;
		let entities = stargate.build_registry::<EntityRendererPrototype>()?;

		for (_, prototype) in block_layers.iter() {
			prototype.get_sprites(&mut sprites);
		}

		for (_, prototype) in entities.iter() {
			prototype.get_sprites(&mut sprites);
		}

		let atlas = Atlas::new(frontend, core, sprites)?;

		Ok(GraphicsRPC {
			block_layer_renderer: server
				.world
				.block_layer
				.zip(block_layers, |_, _, _, parent, prototype| {
					prototype.bake(&core.lua, &atlas, parent)
				})?,
			entity_renderer: server
				.world
				.entity
				.zip(entities, |_, _, _, _, prototype| Ok(prototype.bake(&atlas)))?,
			atlas: Some(atlas),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn compiel() {}
}
