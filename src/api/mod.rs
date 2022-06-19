use std::{collections::HashMap, io, io::ErrorKind, path::PathBuf, sync::Arc};

use eyre::{Context, Result};
use rayon::{ThreadPool, ThreadPoolBuilder};
use tracing::{info};

use crate::{
	api::{
		luna::{lib::reload::Reload, Luna},
		plugin::Plugin,
		registry::Registry,
	},
	multi_deref_fields,
	ty::{identifier::Identifier, MultiDeref},
	util::blake3::{Blake3Hash, Hasher},
	world::{
		chunk::layer::{BlockLayer, BlockLayerPrototype},
		entity::prototype::{EntityDesc, EntityPrototype},
	},
};

use apollo::impl_macro::*;
use apollo::{LuaScope};

pub mod id_table;
pub mod luna;
pub mod plugin;
pub mod prototype;
pub mod registry;
pub mod util;

pub struct Api {
	pub carrier: Carrier,
	pub resources: Plugins,
	pub thread_pool: Arc<ThreadPool>,
	pub luna: Luna,
	pub hash: Option<Blake3Hash>,
}

#[lua_impl]
impl Api {
	#[lua_field(get carrier)]
	pub fn get_carrier(&self) -> &Carrier {
		&self.carrier
	}

	pub fn new(run_dir: PathBuf, extra: Vec<PathBuf>) -> Result<Api> {
		let plugins_path = run_dir.join("./plugins");
		if !plugins_path.exists() {
			std::fs::create_dir_all(&plugins_path).wrap_err("Could not create dirs.")?;
		}

		let mut paths: Vec<PathBuf> = std::fs::read_dir(plugins_path)?
			.flatten()
			.map(|entry| entry.path())
			.collect();
		paths.extend(extra);

		let mut plugins = HashMap::new();
		for path in paths {
			if path.is_dir()
				|| (path.is_file()
					&& path
						.extension()
						.map(|extention| extention.to_str().unwrap() == "zip")
						.unwrap_or(false))
			{
				let plugin = Plugin::new(&path)?;
				plugins.insert(plugin.id.clone(), plugin);
			}
		}

		let resources = Plugins {
			plugins: Arc::new(plugins),
		};
		Ok(Api {
			luna: Luna::new(&resources)?,
			carrier: Carrier {
				block_layer: Registry::default(),
				entity: Registry::default(),
			},
			resources,
			thread_pool: Arc::new(ThreadPoolBuilder::new().build()?),
			hash: None,
		})
	}

	pub fn reload(&mut self, reload: &mut Reload) -> Result<()> {
		self.hash = None;

		// Prepare for reload
		reload.stargate.register_builder::<BlockLayerPrototype>();
		reload.stargate.register_builder::<EntityPrototype>();

		{
			let reload_scope = LuaScope::from(&mut *reload);
			self.luna.lua.globals().set("reload", reload_scope.lua())?;

			for plugin in self.resources.plugins.values() {
				info!("Reloading {}", &plugin.id);
				let identifier = Identifier::new("main.lua");
				let data = self
					.resources
					.get_resource(ResourceKind::Source, &identifier)
					.wrap_err(format!(
						"Could not find entrypoint \"main.lua\" for plugin {}",
						plugin.id
					))?;

			//	let ptr_mut = LuaGluePtrMut::new(reload);
				self.luna.load(&identifier, &data)?.exec()?;
			}
		}

		let registry = reload
			.stargate
			.build_registry::<BlockLayerPrototype>(&self.luna.lua)?;

		let block_layer = registry
			.table
			.into_iter()
			.zip(registry.id_to_ident.into_iter());

		let mut out = Vec::new();
		for ((id, prototype), (_, identifier)) in block_layer {
			out.push((id.build(), identifier, prototype.bake()?));
		}
		let block_layer = out.into_iter().collect();

		self.carrier = Carrier {
			block_layer,
			entity: reload
				.stargate
				.build_registry::<EntityPrototype>(&self.luna.lua)?
				.into_entries()
				.map(|(id, ident, prototype)| (id.build(), ident, prototype.bake(id)))
				.collect(),
		};

		// Hash
		let mut hasher = Hasher::new();
		self.carrier.block_layer.append_hasher(&mut hasher);
		self.carrier.entity.append_hasher(&mut hasher);
		self.hash = Some(hasher.finalize());
		Ok(())
	}
}
pub enum ResourceKind {
	Assets,
	Source,
}

#[derive(Clone)]
pub struct Plugins {
	pub plugins: Arc<HashMap<String, Plugin>>,
}

impl Plugins {
	pub fn get_resource(&self, kind: ResourceKind, location: &Identifier) -> io::Result<Vec<u8>> {
		let plugin = self.plugins.get(&location.namespace).ok_or_else(|| {
			io::Error::new(
				ErrorKind::NotFound,
				format!("Plugin {} does not exist", location.namespace),
			)
		})?;

		let prefix = match kind {
			ResourceKind::Assets => "assets",
			ResourceKind::Source => "src",
		};
		plugin.archive.get(&format!("{}/{}", prefix, location.path))
	}
}

pub struct Carrier {
	pub block_layer: Registry<BlockLayer>,
	pub entity: Registry<EntityDesc>,
}

multi_deref_fields!(Carrier {
	block_layer: Registry<BlockLayer>,
	entity: Registry<EntityDesc>
});

#[lua_impl]
impl Carrier {
	#[lua_field(get block_layer)]
	pub fn get_block_layer(&self) -> &Registry<BlockLayer> {
		&self.block_layer
	}

	#[lua_field(get entity)]
	pub fn get_entity(&self) -> &Registry<EntityDesc> {
		&self.entity
	}
}