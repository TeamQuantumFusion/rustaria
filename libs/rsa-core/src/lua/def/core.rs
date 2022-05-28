use mlua::{Lua, MetaMethod, Value};

use crate::logging::{log, Level};
use crate::lua::PluginLua;

pub fn register(lua: &Lua) -> mlua::Result<()> {
	let globals = lua.globals();
	globals.set("trace", lua.create_function(trace)?)?;
	globals.set("debug", lua.create_function(debug)?)?;
	globals.set("info", lua.create_function(info)?)?;
	globals.set("warn", lua.create_function(warn)?)?;
	globals.set("error", lua.create_function(error)?)?;
	Ok(())
}

fn trace(lua: &Lua, msg: Value) -> mlua::Result<()> {
	event(lua, Level::Trace, msg)
}

fn debug(lua: &Lua, msg: Value) -> mlua::Result<()> {
	event(lua, Level::Debug, msg)
}

fn info(lua: &Lua, msg: Value) -> mlua::Result<()> {
	event(lua, Level::Info, msg)
}

fn warn(lua: &Lua, msg: Value) -> mlua::Result<()> {
	event(lua, Level::Warn, msg)
}

fn error(lua: &Lua, msg: Value) -> mlua::Result<()> {
	event(lua, Level::Error, msg)
}

fn event(lua: &Lua, level: Level, msg: Value) -> mlua::Result<()> {
	let msg = match msg {
		Value::Nil => "nil".to_string(),
		Value::Boolean(value) => value.to_string(),
		Value::Integer(value) => value.to_string(),
		Value::Number(value) => value.to_string(),
		Value::String(string) => string.to_str()?.to_string(),
		Value::UserData(userdata) => {
			if let Value::Function(func) = userdata.get_metatable()?.get(MetaMethod::ToString)? {
				func.call(userdata)?
			} else {
				"no __tostring".to_string()
			}
		}
		_ => "unknown".to_string(),
	};
	let log_target = "plugin_log@".to_owned() + &*PluginLua::import(lua).id;
	let log_target: &str = &log_target;
	log!(target: log_target, level, "{msg}");
	Ok(())
}
