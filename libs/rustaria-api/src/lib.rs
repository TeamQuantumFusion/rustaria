use crate::lua::reload::RegistryBuilderLua;
use crate::registry::RegistryBuilder;
use eyre::{ContextCompat, Result, WrapErr};
use plugin::Plugin;
use registry::Registry;
use rustaria_util::{
	blake3::{Blake3Hash, Hasher},
	debug, info, trace, warn,
};
use std::{
	collections::HashMap,
	io::{self, ErrorKind},
	path::PathBuf,
	sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use ty::{PluginId, Prototype, Tag};
use type_map::concurrent::TypeMap;

mod archive;

pub mod lua;
pub mod plugin;
pub mod registry;
pub mod ty;

pub struct Api {
	plugins: HashMap<PluginId, Plugin>,
}

impl Api {
	pub fn new(plugins_dir: PathBuf, extra_locations: Vec<PathBuf>) -> io::Result<Api> {
		info!(target: "init@rustaria-api", "Loading plugins.");
		let mut plugins = HashMap::new();

		if let Ok(dir) = std::fs::read_dir(plugins_dir) {
			for entry in dir.flatten() {
				Self::load_plugin(entry.path(), &mut plugins);
			}
		}

		for path in extra_locations {
			Self::load_plugin(path, &mut plugins);
		}

		Ok(Api { plugins })
	}

	fn load_plugin(path: PathBuf, plugins: &mut HashMap<String, Plugin>) {
		if match path.extension() {
			Some(extention) if extention == "zip" => true,
			_ => path.is_dir(),
		} {
			match Plugin::new(&path) {
				Ok(plugin) => {
					info!(target: "init@rustaria-api", " - {} [{} {}]", plugin.manifest.name, plugin.manifest.id, plugin.manifest.version);
					plugins.insert(plugin.manifest.id.clone(), plugin);
				}
				Err(error) => {
					warn!("Could not load plugin at {path:?}. Reason: {error:?}");
				}
			}
		}
	}

	pub fn get_asset(&self, location: &Tag) -> io::Result<Vec<u8>> {
		self.plugins
			.get(location.plugin_id())
			.ok_or(ErrorKind::NotFound)?
			.archive
			.get_asset(&("asset/".to_owned() + location.identifier()))
	}

	pub fn reload<'a>(&'a mut self, stack: &'a mut Carrier) -> ApiReload<'a> {
		info!("Locking api.");
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
			hasher: Hasher::new(),
		}
	}
}

pub struct ApiReload<'a> {
	api: &'a mut Api,
	carrier_lock: RwLockWriteGuard<'a, (TypeMap, Blake3Hash)>,
	registry_builders: TypeMap,
	hasher: Hasher,
}

impl<'a> ApiReload<'a> {
	pub fn register<P: Prototype>(&mut self) -> Result<()> {
		debug!(target: "reload",
			"Registered \"{}\" registry.",
			P::lua_registry_name()
		);

		let builder = RegistryBuilderLua::new();
		for (id, plugin) in &self.api.plugins {
			trace!(target: "reload",
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
		for (id, plugin) in &mut self.api.plugins {
			trace!(target: "reload", "Reloading {id}");
			plugin
				.reload()
				.wrap_err(format!("Error while reloading plugin {id}"))?;
		}

		self.carrier_lock.1 = self.hasher.finalize();

		Ok(())
	}

	pub fn collect<P: Prototype>(&mut self) -> Result<()> {
		debug!(target: "reload",
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
		self.carrier_lock.1 = self.hasher.finalize();
	}
}

/// A carrier of all of the registries and the core hash.
/// Carrier has arrived!
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
