use std::collections::HashMap;

use mlua::{Error, FromLua, Lua, Value};
use serde::{Deserialize, Serialize};

use rsa_core::ty::Tag;

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

impl FromLua for Pane {
	fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
		if let Value::Table(table) = value {
			Ok(Pane {
				x_offset: table.get("x_offset")?,
				y_offset: table.get("y_offset")?,
				width: table.get("width")?,
				height: table.get("height")?,
				sprite: table.get("sprite")?,
			})
		} else {
			Err(Error::UserDataTypeMismatch)
		}
	}
}

impl FromLua for RenderingSystem {
	fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
		if let Value::Table(table) = value {
			for (string, value) in table.pairs::<String, Value>().flatten() {
				match string.as_str() {
					"Static" => {
						return Ok(RenderingSystem::Static(FromLua::from_lua(value, lua)?));
					}
					"State" => {
						return Ok(RenderingSystem::State(FromLua::from_lua(value, lua)?));
					}
					_ => {}
				}
			}
		}

		Err(Error::UserDataTypeMismatch)
	}
}
