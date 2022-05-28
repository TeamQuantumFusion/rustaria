extern crate core;

use std::{
	collections::HashMap,
	io::{self, ErrorKind},
	path::PathBuf,
	sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use log::{info, warn};
use mlua::{Lua, ToLuaMulti, UserData};
use type_map::TypeMap;

use crate::api::reload::{LuaReload, Reload};
use carrier::{Carrier, CarrierData};


use crate::error::Result;
use crate::hook::HookInstance;
use crate::plugin::archive::{Archive};
use crate::plugin::Plugin;
use crate::ty::{PluginId, Tag};

pub mod carrier;
pub mod reload;
pub mod lua;

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

	pub fn reload(&mut self) -> Reload {
		Reload::new(self)
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

	pub fn load_simple_plugin(&mut self, code: &str) {
		self.load_test_plugins(vec![Plugin::new_test(
			"hello",
			Archive::new_test(vec![crate::plugin::archive::TestAsset::lua("entry", code)]),
			&self,
		)])
	}
}

#[cfg(any(feature = "test-utils", test))]
mod test_utils {
	#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
	pub struct Counter {
		pub count: u32,
	}

	use apollo::*;
	#[lua_impl]
	impl Counter {
		pub fn new() -> Counter {
			Counter { count: 0 }
		}

		#[lua_method]
		pub fn inc(&mut self) {
			self.count += 1;
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::api::test_utils::Counter;
	use crate::api::Api;
	use crate::error::Result;
	use crate::api::lua::glue::ToGlue;
	use crate::ty::{Prototype, RawId, Tag};
	use crate::{initialize, reload};
	use crate::api::lua::FromLua;

	#[test]
	pub fn test_registry() -> Result<()> {
		initialize()?;

		#[derive(Clone, PartialEq, Debug, FromLua)]
		pub struct FrogePrototype {
			cool: bool,
		}

		impl Prototype for FrogePrototype {
			type Item = Froge;

			fn create(&self, id: RawId) -> Self::Item {
				Froge { cool: self.cool }
			}

			fn lua_registry_name() -> &'static str {
				"froge"
			}
		}

		pub struct Froge {
			cool: bool,
		}

		let mut api = Api::new_test();
		api.load_simple_plugin(
			r#"
			reload.registry["froge"]:insert {
				["frog"] = {
					cool = true
				}
			}
			"#,
		);

		reload!((FrogePrototype) => api);

		assert_eq!(
			api.get_carrier().get::<FrogePrototype>().entries[0],
			FrogePrototype { cool: true }
		);
		Ok(())
	}

	#[test]
	pub fn test_hook() -> Result<()> {
		initialize()?;
		let mut api = Api::new_test();
		api.load_simple_plugin(
			r#"
			reload.hook["r:love_froge"]:subscribe("our_hook", function(var)
				var:inc()
			end)
			"#,
		);

		reload!(() => api);

		let mut counter = Counter::new();
		api.invoke_hook(&Tag::rsa("love_froge"), || counter.glue().lua())?;

		assert_eq!(counter.count, 1);
		Ok(())
	}
}
