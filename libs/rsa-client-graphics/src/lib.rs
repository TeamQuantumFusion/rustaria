#![feature(drain_filter)]

use std::{collections::HashSet, mem::replace};

use apollo::Lua;
use rsa_core::{
	api::{
		reload::{Reload, RustariaPrototypeCarrier},
		stargate::Stargate,
		Core,
	},
	blake3::Hasher,
	err::ext::AuditExt,
	ty::{IdTable, Registry},
};
use rsa_world::{chunk::layer::BlockLayer, entity::prototype::EntityDesc};
use rsa_client_core::atlas::Atlas;
use rustaria::rpc::ServerRPC;
use rsa_core::err::Result;
use rsa_client_core::frontend::Frontend;

use crate::world::{
	chunk::{
		block::BlockRenderer,
		layer::{BlockLayerRenderer, BlockLayerRendererPrototype},
	},
	entity::{EntityRenderer, EntityRendererPrototype},
};

pub mod world;

#[derive(Default)]
pub struct GraphicsRPC {
	pub block_layer_renderer: IdTable<BlockLayer, Option<BlockLayerRenderer>>,
	pub entity_renderer: IdTable<EntityDesc, Option<EntityRenderer>>,
	pub atlas: Option<Atlas>,
}

impl GraphicsRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<BlockLayerRendererPrototype>(lua)?;
		stargate.register_builder::<EntityRendererPrototype>(lua)?;
		Ok(())
	}

	pub fn build(frontend: &Frontend, server: &ServerRPC, core: &Core, stargate: &mut Stargate) -> Result<GraphicsRPC> {
		let mut sprites = HashSet::new();
		let block_layers = stargate.build_registry::<BlockLayerRendererPrototype>()?;
		let entities = stargate.build_registry::<EntityRendererPrototype>()?;

		for (_, prototype) in block_layers.table.iter() {
			prototype.get_sprites(&mut sprites);
		}

		for (_, prototype) in entities.table.iter() {
			prototype.get_sprites(&mut sprites);
		}

		let atlas = Atlas::new(frontend, core, sprites)?;

		let mut block_layer_renderer = Vec::new();
		for (id, _, _) in server.world.block_layer.entries() {
			block_layer_renderer.push((id, None));
		}
		for (_, identifier, prototype) in block_layers.into_entries() {
			if let Some(id) = server.world
				.block_layer
				.get_id_from_identifier(&identifier)
			{
				let prototype = prototype
					.bake(&core.lua, &atlas, server.world.block_layer.get(id))
					.wrap_err_with(|| format!("Failed to bake {}", identifier))?;
				let _ = replace(&mut block_layer_renderer[id.index()], (id, Some(prototype)));
			}
		}

		let mut entity_renderer = Vec::new();
		for (id, _, _) in server.world.entity.entries() {
			entity_renderer.push((id, None));
		}

		for (_, identifier, prototype) in entities.into_entries() {
			if let Some(id) = server.world.entity.get_id_from_identifier(&identifier) {
				let _ = replace(
					&mut entity_renderer[id.index()],
					(id, Some(prototype.bake(&atlas))),
				);
			}
		}

		Ok(GraphicsRPC {
			block_layer_renderer: block_layer_renderer.into_iter().collect(),
			entity_renderer: entity_renderer.into_iter().collect(),
			atlas: Some(atlas)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn compiel() {}
}
