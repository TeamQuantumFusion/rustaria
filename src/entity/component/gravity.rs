use mlua::{Error, FromLua, Lua, Value};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GravityComp {
	pub speed: f32,
}

impl FromLua for GravityComp {
	fn from_lua(lua_value: Value, _: &Lua) -> mlua::Result<Self> {
		if let Value::Number(number) = lua_value {
			Ok(GravityComp {
				speed: number as f32,
			})
		} else {
			Err(Error::RuntimeError("wrong type".to_string()))
		}
	}
}
