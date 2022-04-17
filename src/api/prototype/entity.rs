use mlua::{FromLua, Lua, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use rustaria_api::ty::{Tag};
use rustaria_api::ty::{Prototype, RawId};
use rustaria_util::ty::pos::Pos;

use crate::api::rendering::RenderingSystem;
use crate::entity::VelocityComp;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityPrototype {
	pub velocity: Option<VelocityCompPrototype>,
	#[cfg(feature = "client")]
	pub rendering: Option<RenderingSystem>,
}

impl FromLua for EntityPrototype {
	fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
		if let mlua::Value::Table(table) = value {
			Ok(EntityPrototype {
				velocity: table.get("velocity")?,
				rendering: table.get("rendering")?,
				// collision: table.get("collision")?,
				// opaque: table.get("opaque")?,
				// blast_resistance: table.get("blast_resistance")?,
				// break_resistance: table.get("break_resistance")?,
			})
		} else {
			Err(mlua::Error::UserDataTypeMismatch)
		}
	}
}

impl Prototype for EntityPrototype {
	type Item = ();

	fn create(&self, _: RawId) -> Self::Item {}

	fn get_sprites(&self, sprites: &mut HashSet<Tag>) {
		if let Some(system) = &self.rendering {
			match system {
				RenderingSystem::Static(pane) => {
					sprites.insert(pane.sprite.clone());
				}
				RenderingSystem::State(states) => {
					for pane in states.values() {
						sprites.insert(pane.sprite.clone());
					}
				}
			}
		}
	}

	fn lua_registry_name() -> &'static str {
		"Entities"
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct VelocityCompPrototype {
	pub x: f32,
	pub y: f32,
}

impl Prototype for VelocityCompPrototype {
	type Item = VelocityComp;

	fn create(&self, _: RawId) -> Self::Item {
		VelocityComp {
			velocity: Pos {
				x: self.x,
				y: self.y,
			},
		}
	}
}

impl FromLua for VelocityCompPrototype {
	fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
		if let mlua::Value::Table(table) = value {
			Ok(VelocityCompPrototype {
				x: table.get("x")?,
				y: table.get("y")?,
			})
		} else {
			Err(mlua::Error::UserDataTypeMismatch)
		}
	}
}
