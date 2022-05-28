use mlua::{Lua, MetaMethod, Table, Value};
use crate::api::lua::get_meta;

use crate::logging::{log, Level};

pub fn register(lua: &Lua, globals: &Table) -> mlua::Result<()> {
	let log = lua.create_table()?;
	log.set("trace", lua.create_function(trace)?)?;
	log.set("debug", lua.create_function(debug)?)?;
	log.set("info", lua.create_function(info)?)?;
	log.set("warn", lua.create_function(warn)?)?;
	log.set("error", lua.create_function(error)?)?;
	globals.set("log", log)?;
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
	let meta = get_meta(lua);

	let log_target = "plugin_log@".to_owned() + &meta.plugin_id;
	let log_target: &str = &log_target;
	log!(target: log_target, level, "{msg}");
	Ok(())
}
