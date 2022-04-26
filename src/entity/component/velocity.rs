use mlua::{FromLua, Lua, Value};
use serde::Deserialize;

use rustaria_common::logging::info;
use rustaria_common::math::{Vector2D, WorldSpace};
use rustaria_common::ty::Pos;

use crate::UPS;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct PhysicsComp {
	pub velocity: Vector2D<f32, WorldSpace>,
	pub acceleration: Vector2D<f32, WorldSpace>,
}

impl PhysicsComp {
	pub fn tick(&mut self) {
		self.velocity += self.acceleration / UPS as f32;
		let velocity = self.velocity;
		let acceleration = self.acceleration;
		info!("{velocity:?}:{acceleration:?}");
	}
}

impl FromLua for PhysicsComp {
	fn from_lua(lua_value: Value, lua: &Lua) -> mlua::Result<Self> {
		Ok(PhysicsComp {
			velocity: Pos::from_lua(lua_value, lua)?.into(),
			acceleration: Default::default(),
		})
	}
}
