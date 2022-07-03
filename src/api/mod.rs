use std::{collections::HashMap, io, io::ErrorKind, path::PathBuf, sync::Arc};

use anyways::{ext::AuditExt, Result};
use rayon::{ThreadPool, ThreadPoolBuilder};

use apollo::{Lua, LuaScope, macros::*};

use crate::{
	api::{
		plugin::Plugin,
		registry::Registry,
	},
	item::{ItemDesc},
	multi_deref_fields,
	ty::{identifier::Identifier, MultiDeref},
	util::blake3::{Blake3Hash, Hasher},
	world::{
		chunk::layer::{BlockLayer, BlockLayerPrototype},
		entity::prototype::{EntityDesc, EntityPrototype},
	},
};
use crate::api::lua::create_lua;
use crate::api::lua::lib::reload::Reload;
use crate::api::lua::lib::stargate::Stargate;
use crate::item::prototype::ItemPrototype;

pub mod id_table;
pub mod plugin;
pub mod prototype;
pub mod registry;
pub mod util;
pub mod lua;

pub struct Api {
	pub carrier: Carrier,
	pub plugins: Plugins,
	pub thread_pool: Arc<ThreadPool>,
	pub hash: Option<Blake3Hash>,
	pub lua: Lua,
}


#[lua_impl]
impl Api {
	#[lua_field(get carrier)]
	pub fn get_carrier(&self) -> &Carrier { &self.carrier }

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

		let plugins = Plugins {
			plugins: Arc::new(plugins),
		};

		Ok(Api {
			lua: create_lua(&plugins).wrap_err("Failed to initialize lua")?,
			carrier: Carrier {
				block_layer: Registry::default(),
				entity: Registry::default(),
				item: Registry::default()
			},
			plugins,
			thread_pool: Arc::new(ThreadPoolBuilder::new().build()?),
			hash: None,
		})
	}

	pub fn reload(&mut self, reload: &mut Reload) -> Result<()> {
		self.hash = None;

		// Prepare for reload
		reload
			.stargate
			.register_builder::<BlockLayerPrototype>(&self.lua)?;
		reload
			.stargate
			.register_builder::<EntityPrototype>(&self.lua)?;
		reload
			.stargate
			.register_builder::<ItemPrototype>(&self.lua)?;

		{
			let reload_scope = LuaScope::from(&mut *reload);
			self
				.lua
				.globals()
				.insert("reload", reload_scope.lua())
				.wrap_err("Failed to insert reload")?;

			for plugin in self.plugins.plugins.values() {
				plugin
					.reload(&self.lua)
					.wrap_err_with(|| format!("Failed to reload plugin {}", plugin.id))?;
			}
		}

		let registry = reload
			.stargate
			.build_registry::<BlockLayerPrototype>(&self.lua)?;

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
				.build_registry::<EntityPrototype>(&self.lua)?
				.into_entries()
				.map(|(id, ident, prototype)| (id.build(), ident, prototype.bake(id)))
				.collect(),
			item: reload
				.stargate
				.build_registry::<ItemPrototype>(&self.lua)?
				.into_entries()
				.map(|(id, ident, prototype)| (id.build(), ident, prototype.bake()))
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

#[cfg(feature = "testing")]
impl Api {
	pub fn test_simple(entrypoint: String) -> Api {
		let mut api = Api::new_test(vec![Plugin::test(entrypoint)]);
		api.reload(&mut Reload {
			stargate: Stargate::new(),
			client: false,
		}).unwrap();
		api
	}

	pub fn new_test(plugins: Vec<Plugin>) -> Api {
		let plugins = Plugins {
			plugins: Arc::new(
				plugins
					.into_iter()
					.map(|p| (p.id.clone(), p))
					.collect()
			)
		};
		Api {
			carrier: Carrier {
				block_layer: Default::default(),
				entity: Default::default(),
				item: Default::default()
			},
			lua: create_lua(&plugins).unwrap(),
			plugins,
			thread_pool: Arc::new(ThreadPoolBuilder::new().num_threads(1).build().unwrap()),
			hash: None
		}
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
	pub item: Registry<ItemDesc>,
}

multi_deref_fields!(Carrier {
	block_layer: Registry<BlockLayer>,
	entity: Registry<EntityDesc>,
	item: Registry<ItemDesc>
});

#[lua_impl]
impl Carrier {
	#[lua_field(get block_layer)]
	pub fn get_block_layer(&self) -> &Registry<BlockLayer> { &self.block_layer }

	#[lua_field(get entity)]
	pub fn get_entity(&self) -> &Registry<EntityDesc> { &self.entity }

	#[lua_field(get item)]
	pub fn get_item(&self) -> &Registry<ItemDesc> { &self.item }
}
