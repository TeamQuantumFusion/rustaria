use std::{collections::HashMap, fmt::Write, hash::Hash, path::PathBuf, sync::Arc};

use anyways::ext::AuditExt;
use apollo::{prelude::LuaError, Lua, LuaScope};
use log::debug;

use crate::{
	blake3::Hasher, err::Result, ty::Identifier, Blake3Hash,
	Plugin, Plugins, Reload, Stargate, ThreadPool, ThreadPoolBuilder,
};

pub mod lua;
pub mod plugin;
pub mod prototype;
pub mod reload;
pub mod stargate;
pub mod util;

pub struct Core {
	pub plugins: Plugins,
	pub thread_pool: Arc<ThreadPool>,
	pub hash: Option<Blake3Hash>,
	pub lua: Lua,
}

impl Core {
	pub fn new(run_dir: PathBuf, extra: Vec<PathBuf>) -> Result<Core> {
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

		Ok(Core {
			lua: create_lua(&plugins).wrap_err("Failed to initialize lua")?,
			plugins,
			thread_pool: Arc::new(ThreadPoolBuilder::new().build()?),
			hash: None,
		})
	}

	pub fn reload(&mut self, reload: &mut Reload) -> Result<()> {
		self.hash = None;
		{
			let reload_scope = LuaScope::from(&mut *reload);
			self.lua
				.globals()
				.insert("reload", reload_scope.lua())
				.wrap_err("Failed to insert reload")?;

			for plugin in self.plugins.plugins.values() {
				plugin
					.reload(&self.lua)
					.wrap_err_with(|| format!("Failed to reload plugin {}", plugin.id))?;
			}
		}
		Ok(())
	}
}

#[cfg(feature = "testing")]
impl Core {
	pub fn test_simple(entrypoint: String) -> Core {
		Core::new_test(vec![Plugin::test(entrypoint)])
	}

	pub fn new_test(plugins: Vec<Plugin>) -> Core {
		let plugins = Plugins {
			plugins: Arc::new(plugins.into_iter().map(|p| (p.id.clone(), p)).collect()),
		};
		Core {
			lua: create_lua(&plugins).unwrap(),
			plugins,
			thread_pool: Arc::new(ThreadPoolBuilder::new().num_threads(1).build().unwrap()),
			hash: None,
		}
	}
}

pub fn create_lua(plugins: &Plugins) -> Result<Lua> {
	let lua = Lua::new();
	lua::register(&lua).wrap_err("Failed to initialize rustaria-lua library")?;

	// Setup the loader to allow plugins to have multiple files
	let plugins = plugins.clone();
	lua.set_loader(move |lua, location| {
		let mut location = Identifier::new_lua(location)?;
		location
			.path
			.write_str(".lua")
			.map_err(LuaError::external)?;
		debug!(target: "luna::loading", "Loading {}", location);

		let data = plugins.get_resource(ResourceKind::Source, &location)?;
		lua.load(&data)
			.set_name(format!("{location}"))?
			.into_function()
	})
	.wrap_err("Failed to initialize file loader.")?;

	Ok(lua)
}

pub enum ResourceKind {
	Assets,
	Source,
}
