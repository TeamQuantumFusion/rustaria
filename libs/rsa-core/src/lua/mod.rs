use log::debug;
pub use mlua::prelude::*;
pub use mlua::Lua;
use mlua::{Result, StdLib, Table, UserData};

use crate::api::{Api, AssetKind};
use crate::plugin::Manifest;
use crate::ty::Tag;

pub mod def;
pub mod glue;
pub mod error;

pub mod util {
	pub use frogelua::*;
}

pub fn new_lua(manifest: &Manifest, api: &Api) -> Result<Lua> {
	let lua_state = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new().catch_rust_panics(false))?;
	let globals = lua_state.globals();

	globals.set("_api", api.clone())?;
	globals.set(
		"plugin",
		PluginLua {
			id: manifest.id.clone(),
		},
	)?;

	// Overwrite module loading
	let package: Table = globals.get("package")?;
	package.set("path", "your mom")?;
	let searchers: Table = package.get("loaders")?;
	searchers.raw_insert(
		2,
		lua_state.create_function(|lua, location: Tag| {
			debug!(target: "misc@rustaria.api", "Loading {}", location);
			let api: Api = lua
				.globals()
				.get("_api")
				.expect("Api module global is missing.");

			let asset = api.get_asset(AssetKind::Source, &location)?;
			lua.load(&asset).into_function()
		})?,
	)?;

	def::core::register(&lua_state)?;

	Ok(lua_state)
}

pub fn get_api(lua: &Lua) -> Api {
	lua.globals().get("_api").expect("The reserved _api global is missing.")
}

#[derive(Clone)]
pub struct PluginLua {
	pub id: String,
}

impl UserData for PluginLua {}

impl PluginLua {
	pub fn import(lua: &Lua) -> PluginLua {
		lua.globals()
			.get("plugin")
			.expect("Could not get plugin global.")
	}
}
