use crate::lua::PluginLua;

use crate::Tag;
use mlua::Lua;
use rustaria_util::{log, Level};

pub fn register(lua: &Lua) -> mlua::Result<()> {
	let globals = lua.globals();
	globals.set("trace", lua.create_function(trace)?)?;
	globals.set("debug", lua.create_function(debug)?)?;
	globals.set("info", lua.create_function(info)?)?;
	globals.set("warn", lua.create_function(warn)?)?;
	globals.set("error", lua.create_function(error)?)?;
	Ok(())
}

fn trace(lua: &Lua, msg: String) -> mlua::Result<()> {
	event(lua, Level::Trace, msg)
}

fn debug(lua: &Lua, msg: String) -> mlua::Result<()> {
	event(lua, Level::Debug, msg)
}

fn info(lua: &Lua, msg: String) -> mlua::Result<()> {
	event(lua, Level::Info, msg)
}

fn warn(lua: &Lua, msg: String) -> mlua::Result<()> {
	event(lua, Level::Warn, msg)
}

fn error(lua: &Lua, msg: String) -> mlua::Result<()> {
	event(lua, Level::Error, msg)
}

fn event(lua: &Lua, level: Level, msg: String) -> mlua::Result<()> {
	let log_target = "plugin_log@".to_owned() + &*PluginLua::import(lua).id;
	let log_target: &str = &log_target;
	log!(target: log_target, level, "{msg}");
	Ok(())
}
