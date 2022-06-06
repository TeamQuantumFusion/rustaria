use mlua::{FromLua, Lua, Value};
use serde::Deserialize;

use rsa_core::math::{Vector2D, WorldSpace};
use rsa_core::settings::UPS;
use rsa_core::ty::Pos;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct PhysicsComp {
	pub velocity: Vector2D<f32, WorldSpace>,
	pub acceleration: Vector2D<f32, WorldSpace>,
}

impl PhysicsComp {
	pub fn tick(&mut self) {
		self.velocity += self.acceleration / UPS as f32;
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
