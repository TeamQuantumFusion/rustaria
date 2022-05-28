extern crate core;

use std::{
	collections::HashMap,
	io::{self, ErrorKind},
	path::PathBuf,
	sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use log::{info, warn};
use mlua::{ToLuaMulti, UserData};
use type_map::TypeMap;

use carrier::{Carrier, CarrierData};
use reload::ApiReload;

use crate::blake3::Hasher;
use crate::lua::def::hook::HookInstance;
use crate::plugin::Plugin;
use crate::ty::{PluginId, Tag};
use crate::error::Result;

pub mod carrier;
pub mod reload;

#[derive(Clone)]
pub struct Api {
	internals: Arc<RwLock<ApiInternals>>,
}

impl Api {
	pub fn new(plugins_dir: PathBuf, extra_locations: Vec<PathBuf>) -> io::Result<Api> {
		let api = Api {
			internals: Arc::new(RwLock::new(ApiInternals {
				plugins: Default::default(),
				carrier: Carrier::new(),
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

	pub fn get_carrier(&self) -> Carrier {
		self.internals.read().unwrap().carrier.clone()
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
		name: &Tag,
		args_func: F,
	) -> Result<()> {
		let guard = self.read();
		let instance = &guard.hook_instance;
		instance.trigger(name, args_func)
	}

	pub fn reload(&mut self) -> ApiReload {
		info!(target: "init@rustaria.api", "Freezing carrier.");
		let carrier = unsafe { &mut *(self.get_carrier().data.get() as *const CarrierData as *mut CarrierData) };

		carrier.registries.clear();
		carrier.hash = Default::default();

		ApiReload {
			api: self,
			carrier,
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
	carrier: Carrier,
	hook_instance: HookInstance,
}

impl UserData for Api {}

pub trait Reloadable {
	fn reload(&mut self, api: &Api);
}

#[cfg(any(feature = "test-utils", test))]
impl Api {
	pub fn new_test() -> Api {
		Api {
			internals: Arc::new(RwLock::new(ApiInternals {
				plugins: Default::default(),
				carrier: Carrier::new(),
				hook_instance: Default::default(),
			})),
		}
	}

	pub fn load_test_plugins(&mut self, plugins: Vec<Plugin>) {
		self.internals.write().unwrap().plugins = plugins
			.into_iter()
			.map(|plugin| (plugin.manifest.id.clone(), plugin))
			.collect();
	}
}

#[cfg(test)]
mod tests {
	use crate::api::Api;
	use crate::plugin::archive::{Archive, TestAsset};
	use crate::plugin::Plugin;

	#[test]
	pub fn test() {
		let mut api = Api::new_test();
		api.load_test_plugins(vec![Plugin::new_test(
			"hello",
			Archive::new_test(vec![TestAsset::lua(
				"entry",
				r#"
				-- entry stuff


				"#,
			)]),
			&api,
		)]);
	}
}
