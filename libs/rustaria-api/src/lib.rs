extern crate core;

use std::{
	collections::HashMap,
	io::{self, ErrorKind},
	path::PathBuf,
	sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use mlua::{ToLuaMulti, UserData, Value};
use type_map::concurrent::TypeMap;

use plugin::Plugin;
use registry::Registry;
use rustaria_util::blake3::{Blake3Hash, Hasher};
use rustaria_util::error::{ContextCompat, Result, WrapErr};
use rustaria_util::logging::{debug, info, trace, warn};
use ty::{PluginId, Prototype, Tag};

use crate::lua::hook::{HookInstance, HookInstanceBuilder};
use crate::lua::reload::RegistryBuilderLua;
use crate::registry::RegistryBuilder;

mod archive;

pub mod lua;
pub mod plugin;
pub mod registry;
pub mod ty;

#[derive(Clone)]
pub struct Api {
	internals: Arc<RwLock<ApiInternals>>,
}

impl Api {
	pub fn new(plugins_dir: PathBuf, extra_locations: Vec<PathBuf>) -> io::Result<Api> {
		let api = Api {
			internals: Arc::new(RwLock::new(ApiInternals {
				plugins: Default::default(),
				hook_instance: HookInstance::default(),
			})),
		};

		info!(target: "init@rustaria.api", "Loading plugins.");
		if let Ok(dir) = std::fs::read_dir(plugins_dir) {
			for entry in dir.flatten() {
				Self::load_plugin(entry.path(), &api);
			}
		}

		for path in extra_locations {
			Self::load_plugin(path, &api);
		}

		Ok(api)
	}

	pub(crate) fn read(&self) -> RwLockReadGuard<'_, ApiInternals> {
		self.internals.read().unwrap()
	}

	pub(crate) fn write(&self) -> RwLockWriteGuard<'_, ApiInternals> {
		self.internals.write().unwrap()
	}

	fn load_plugin(path: PathBuf, api: &Api) {
		if match path.extension() {
			Some(extention) if extention == "zip" => true,
			_ => path.is_dir(),
		} {
			match Plugin::new(&path, api) {
				Ok(plugin) => {
					info!(target: "init@rustaria.api", " - {} [{} {}]", plugin.manifest.name, plugin.manifest.id, plugin.manifest.version);
					api.write()
						.plugins
						.insert(plugin.manifest.id.clone(), plugin);
				}
				Err(error) => {
					warn!(target: "init@rustaria.api", "Could not load plugin at {path:?}. Reason: {error:?}");
				}
			}
		}
	}

	pub fn get_asset(&self, kind: AssetKind, location: &Tag) -> io::Result<Vec<u8>> {
		self.internals
			.read()
			.unwrap()
			.plugins
			.get(location.plugin_id())
			.ok_or(ErrorKind::NotFound)?
			.archive
			.get_asset(&(kind.string() + location.identifier()))
	}

	pub fn invoke_hook<F: FnOnce() -> A, A: ToLuaMulti + Clone>(
		&self,
		name: &'static str,
		args_func: F,
	) -> mlua::Result<()> {
		let guard = self.read();
		let instance = &guard.hook_instance;
		instance.trigger(name, args_func)
	}

	pub fn reload<'a>(&'a mut self, stack: &'a mut Carrier) -> ApiReload<'a> {
		info!(target: "init@rustaria.api", "Freezing carrier.");
		let mut lock = stack
			.data
			.write()
			.expect("Could not acquire write lock of RegistryStack.");
		lock.0.clear();
		lock.1 = Default::default();

		ApiReload {
			api: self,
			carrier_lock: lock,
			registry_builders: TypeMap::new(),
			hook_builder: Default::default(),
			hasher: Hasher::new(),
		}
	}
}

pub enum AssetKind {
	Asset,
	Source,
}

impl AssetKind {
	pub fn string(self) -> String {
		match self {
			AssetKind::Asset => "asset/".to_owned(),
			AssetKind::Source => "src/".to_owned(),
		}
	}
}

pub(crate) struct ApiInternals {
	plugins: HashMap<PluginId, Plugin>,
	hook_instance: HookInstance,
}

impl UserData for Api {}

pub struct ApiReload<'a> {
	api: &'a mut Api,
	carrier_lock: RwLockWriteGuard<'a, (TypeMap, Blake3Hash)>,
	registry_builders: TypeMap,

	hook_builder: HookInstanceBuilder,
	hasher: Hasher,
}

impl<'a> ApiReload<'a> {
	pub fn register<P: Prototype>(&mut self) -> Result<()> {
		debug!(target: "reload@rustaria.api",
			"Registered \"{}\" registry.",
			P::lua_registry_name()
		);

		let builder = RegistryBuilderLua::new();
		for (id, plugin) in &self.api.internals.read().unwrap().plugins {
			trace!(target: "reload@rustaria.api",
				"Registered \"{}\" registry to {id}.",
				P::lua_registry_name()
			);
			plugin
				.lua_state
				.globals()
				.set(P::lua_registry_name(), builder.clone())?;
		}

		self.registry_builders
			.insert::<RegistryBuilderLua<P>>(builder);
		Ok(())
	}

	pub fn reload(&mut self) -> Result<()> {
		for (id, plugin) in &self.api.internals.read().unwrap().plugins {
			plugin
				.lua_state
				.globals()
				.set("hook", self.hook_builder.lua())?;

			trace!(target: "reload@rustaria.api", "Reloading {id}");
			plugin
				.reload()
				.wrap_err(format!("Error while reloading plugin {id}"))?;

			plugin.lua_state.globals().set("hook", Value::Nil)?;
		}

		self.carrier_lock.1 = self.hasher.finalize();

		Ok(())
	}

	pub fn collect<P: Prototype>(&mut self) -> Result<()> {
		debug!(target: "reload@rustaria.api",
			"Collecting \"{}\" registry.",
			P::lua_registry_name()
		);

		let builder = self
			.registry_builders
			.remove::<RegistryBuilderLua<P>>()
			.wrap_err(format!(
				"Could not find registry {}",
				P::lua_registry_name()
			))?;

		let registry = builder.collect(&mut self.hasher)?;

		self.carrier_lock.0.insert::<Registry<P>>(registry);
		Ok(())
	}

	pub fn apply(mut self) {
		self.api.write().hook_instance = self.hook_builder.export();
		self.carrier_lock.1 = self.hasher.finalize();
	}
}

/// # Carrier has arrived!
/// A carrier of all of the registries and the core hash.
#[derive(Clone, Default)]
pub struct Carrier {
	data: Arc<RwLock<(TypeMap, Blake3Hash)>>,
}

impl Carrier {
	pub fn lock(&self) -> RegistryStackAccess {
		RegistryStackAccess {
			lock: self.data.read().unwrap(),
		}
	}
}

pub struct RegistryStackAccess<'a> {
	lock: RwLockReadGuard<'a, (TypeMap, [u8; 32])>,
}

impl<'a> RegistryStackAccess<'a> {
	pub fn get_hash(&self) -> [u8; 32] {
		self.lock.1
	}

	// needs to be func because of lock issues
	pub fn get_registry<P: Prototype>(&self) -> &Registry<P> {
		self.lock
			.0
			.get::<Registry<P>>()
			.expect("Could not find registry")
	}
}

pub trait Reloadable {
	fn reload(&mut self, api: &Api, carrier: &Carrier);
}
