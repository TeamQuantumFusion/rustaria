use std::fmt::Write;
use anyways::ext::AuditExt;
use apollo::Lua;
use anyways::Result;
use log::debug;
use apollo::prelude::LuaError;
use crate::api::{Plugins, ResourceKind};
use crate::ty::identifier::Identifier;

pub mod lib;
pub mod table;

pub fn create_lua(plugins: &Plugins) -> Result<Lua> {
	let lua = Lua::new();
	lib::register(&lua).wrap_err("Failed to initialize rustaria-lua library")?;

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
			lua.load(&data).set_name(format!("{location}"))?.into_function()
		}
	).wrap_err("Failed to initialize file loader.")?;

	Ok(lua)
}