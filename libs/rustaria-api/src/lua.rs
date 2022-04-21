use mlua::Result;
use mlua::Table;
use mlua::UserData;
use mlua::{Lua, Value};

use crate::plugin::Manifest;
use crate::{debug, info, Api};
use crate::{AssetKind, Tag};

pub mod core;
pub mod reload;

pub fn new_lua(manifest: &Manifest, api: &Api) -> Result<Lua> {
	let lua_state = Lua::new();
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
	let searchers: Table = package.get("searchers")?;
	searchers.raw_insert(
		2,
		lua_state.create_function(|lua, location: Tag| {
			debug!(target: "misc@rustaria.api", "Loading {}", location);
			let api: Api = lua
				.globals()
				.get("_api")
				.expect("Api internal global is missing.");

			let asset = api.get_asset(AssetKind::Source, &location)?;
			lua.load(&asset).into_function()
		})?,
	)?;

	core::register(&lua_state)?;

	Ok(lua_state)
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
