use std::{collections::HashSet, mem::replace, ops::Deref, path::PathBuf};

use eyre::{Context, Result};
use rustaria::{
	api::{
		id_table::IdTable,
		luna::lib::{reload::Reload, stargate::Stargate},
		Api,
	},
	world::{chunk::layer::BlockLayer, entity::prototype::EntityDesc},
};

use crate::{
	render::{
		atlas::Atlas,
		world::{
			chunk::layer::BlockLayerRendererPrototype,
			entity::{EntityRenderer, EntityRendererPrototype},
		},
	},
	BlockLayerRenderer, Frontend,
};

pub struct ClientApi {
	pub c_carrier: ClientCarrier,
	pub api: Api,
	pub atlas: Option<Atlas>,
}

impl ClientApi {
	pub fn new(run_dir: PathBuf, extra: Vec<PathBuf>) -> Result<ClientApi> {
		Ok(ClientApi {
			api: Api::new(run_dir, extra).wrap_err("Failed to reload common API")?,
			c_carrier: ClientCarrier {
				block_layer_renderer: Default::default(),
				entity_renderer: Default::default(),
			},
			atlas: None,
		})
	}

	pub fn reload(&mut self, frontend: &Frontend) -> Result<()> {
		let mut reload = Reload {
			stargate: Stargate::new(),
			client: true,
		};
		// Register client only prototypes
		reload
			.stargate
			.register_builder::<BlockLayerRendererPrototype>();
		reload
			.stargate
			.register_builder::<EntityRendererPrototype>();

		// reload server stuff
		self.api.reload(&mut reload).wrap_err("Failed to reload")?;

		let mut sprites = HashSet::new();
		let block_layers = reload
			.stargate
			.build_registry::<BlockLayerRendererPrototype>(&self.api.luna.lua)?;

		let entities = reload
			.stargate
			.build_registry::<EntityRendererPrototype>(&self.api.luna.lua)?;

		for (_, prototype) in block_layers.table.iter() {
			prototype.get_sprites(&mut sprites);
		}

		for (_, prototype) in entities.table.iter() {
			prototype.get_sprites(&mut sprites);
		}

		let atlas = Atlas::new(frontend, self, sprites)?;

		let mut block_layer_renderer = Vec::new();
		for (id, _, _) in self.api.carrier.block_layer.entries() {
			block_layer_renderer.push((id, None));
		}
		for (_, identifier, prototype) in block_layers.into_entries() {
			if let Some(id) = self.api.carrier.block_layer.get_id(&identifier) {
				let prototype = prototype
					.bake(
						&self.api.luna.lua,
						&atlas,
						self.api.carrier.block_layer.get(id),
					)
					.wrap_err_with(|| format!("Failed to bake {}", identifier))?;
				let _ = replace(&mut block_layer_renderer[id.index()], (id, Some(prototype)));
			}
		}

		let mut entity_renderer = Vec::new();
		for (id, _, _) in self.api.carrier.entity.entries() {
			entity_renderer.push((id, None));
		}

		for (_, identifier, prototype) in entities.into_entries() {
			if let Some(id) = self.api.carrier.entity.get_id(&identifier) {
				let _ = replace(
					&mut entity_renderer[id.index()],
					(id, Some(prototype.bake(&atlas))),
				);
			}
		}

		self.atlas = Some(atlas);
		self.c_carrier = ClientCarrier {
			block_layer_renderer: block_layer_renderer.into_iter().collect(),
			entity_renderer: entity_renderer.into_iter().collect(),
		};

		Ok(())
	}
}

impl Deref for ClientApi {
	type Target = Api;

	fn deref(&self) -> &Self::Target { &self.api }
}

pub struct ClientCarrier {
	pub block_layer_renderer: IdTable<BlockLayer, Option<BlockLayerRenderer>>,
	pub entity_renderer: IdTable<EntityDesc, Option<EntityRenderer>>,
}
