use mlua::prelude::*;
use rustaria_util::{debug, error, info, trace, warn};

use crate::lua::ctx;

pub fn register(lua: &Lua, _: ()) -> LuaResult<LuaTable<'_>> {
	lua.create_table_from([
		("trace", lua.create_function(trace)?),
		("debug", lua.create_function(debug)?),
		("info", lua.create_function(info)?),
		("warn", lua.create_function(warn)?),
		("error", lua.create_function(error)?),
	])
}

fn trace(lua: &Lua, msg: String) -> LuaResult<()> {
	trace!(target: &ctx(lua).id, "{msg}");
	Ok(())
}

fn debug(lua: &Lua, msg: String) -> LuaResult<()> {
	debug!(target: &ctx(lua).id, "{msg}");
	Ok(())
}

fn info(lua: &Lua, msg: String) -> LuaResult<()> {
	info!(target: &ctx(lua).id, "{msg}");
	Ok(())
}

fn warn(lua: &Lua, msg: String) -> LuaResult<()> {
	warn!(target: &ctx(lua).id, "{msg}");
	Ok(())
}

fn error(lua: &Lua, msg: String) -> LuaResult<()> {
	error!(target: &ctx(lua).id, "{msg}");
	Ok(())
}
