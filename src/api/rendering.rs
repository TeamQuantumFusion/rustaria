use std::collections::HashMap;

use mlua::{Error, Lua, LuaSerdeExt, UserData, Value};
use serde::{Deserialize, Serialize};

use rustaria_api::ty::{LuaConvertableCar, Tag};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RenderingSystem {
	Static(Pane),
	State(HashMap<String, Pane>),
	// More implementations for dynamic lua rendering.
	// Advanced(stuff)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pane {
	pub x_offset: f32,
	pub y_offset: f32,
	pub width: f32,
	pub height: f32,
	pub sprite: Tag,
}

impl LuaConvertableCar for Pane {
	fn from_luaagh(value: Value, lua: &Lua) -> mlua::Result<Self> {
		if let Value::Table(table) = value {
			Ok(Pane {
				x_offset: LuaConvertableCar::from_luaagh(table.get("x_offset")?, lua)?,
				y_offset: LuaConvertableCar::from_luaagh(table.get("y_offset")?, lua)?,
				width: LuaConvertableCar::from_luaagh(table.get("width")?, lua)?,
				height: LuaConvertableCar::from_luaagh(table.get("height")?, lua)?,
				sprite: LuaConvertableCar::from_luaagh(table.get("sprite")?, lua)?,
			})
		} else {
			Err(Error::UserDataTypeMismatch)
		}
	}

	fn into_luaagh(self, lua: &Lua) -> mlua::Result<Value> {
		todo!()
	}
}

impl LuaConvertableCar for RenderingSystem {
	fn from_luaagh(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
		if let Value::Table(table) = value {
			if let value @ Value::Table(_) = table.get("Static")? {
				return Ok(RenderingSystem::Static(Pane::from_luaagh(value, lua)?));
			}
			if let value @ Value::Table(_) = table.get("State")? {
				return Ok(RenderingSystem::State(HashMap::from_luaagh(value, lua)?));
			}
		}

		Err(Error::UserDataTypeMismatch)
	}

	fn into_luaagh(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
		lua.to_value(&self)
	}
}
