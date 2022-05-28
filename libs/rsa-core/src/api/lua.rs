use std::fmt::Write;
use crate::api::{Api, AssetKind};
use crate::plugin::Manifest;
use crate::ty::Tag;
use log::debug;

pub mod error;
pub mod glue;
pub mod lib;

// Reexport
pub use mlua::prelude::*;
pub use mlua::Lua;
pub use frogelua::FromLua;
pub use frogelua::ToLua;

use mlua::{StdLib, UserData};

#[derive(Clone)]
pub struct Metadata {
	pub api: Api,
	pub plugin_id: String,
}

impl UserData for Metadata {

}

pub fn new_lua_state(manifest: &Manifest, api: &Api) -> mlua::Result<Lua> {
	let lua_state = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new().catch_rust_panics(false))?;
	let globals = lua_state.globals();

	globals.set(
		"_meta",
		Metadata {
			api: api.clone(),
			plugin_id: manifest.id.clone(),
		},
	)?;

	// Overwrite module loading
	let package: LuaTable = globals.get("package")?;
	let searchers: LuaTable = package.get("loaders")?;
	searchers.raw_insert(
		2,
		lua_state.create_function(|lua, mut location: Tag| {
			location.inner.write_str(".lua").map_err(|io| LuaError::external(io))?;
			debug!(target: "misc@rustaria.api", "Loading {}", location);
			let meta = get_meta(lua);
			let asset = meta.api.get_asset(AssetKind::Source, &location)?;
			lua.load(&asset).into_function()
		})?,
	)?;

	lib::load_builtin(&lua_state)?;
	Ok(lua_state)
}

pub fn get_meta(lua: &Lua) -> Metadata {
	lua.globals()
		.get("_meta")
		.expect("The reserved _ctx global which holds rustaria metadata is missing.")
}
