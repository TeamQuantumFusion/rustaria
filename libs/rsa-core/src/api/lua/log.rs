use apollo::{Lua, MetaMethod, Table, Value};
use log::{debug, error, info, trace, warn};

pub fn register(lua: &Lua, globals: &Table) -> anyways::Result<()> {
	let log = lua.create_table()?;
	log.insert("trace", lua.create_function(trace)?)?;
	log.insert("debug", lua.create_function(debug)?)?;
	log.insert("info", lua.create_function(info)?)?;
	log.insert("warn", lua.create_function(warn)?)?;
	log.insert("error", lua.create_function(error)?)?;
	globals.insert("log", log)?;
	Ok(())
}

fn trace(_lua: &Lua, msg: Value) -> apollo::Result<()> {
	trace!(target: "luna", "{}", to_string(msg)?);
	Ok(())
}

fn debug(_lua: &Lua, msg: Value) -> apollo::Result<()> {
	debug!(target: "luna", "{}", to_string(msg)?);
	Ok(())
}

fn info(_lua: &Lua, msg: Value) -> apollo::Result<()> {
	info!(target: "luna", "{}", to_string(msg)?);
	Ok(())
}

fn warn(_lua: &Lua, msg: Value) -> apollo::Result<()> {
	warn!(target: "luna", "{}", to_string(msg)?);
	Ok(())
}

fn error(_lua: &Lua, msg: Value) -> apollo::Result<()> {
	error!(target: "luna", "{}", to_string(msg)?);
	Ok(())
}

fn to_string(msg: Value) -> apollo::Result<String> {
	Ok(match msg {
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
	})
}
