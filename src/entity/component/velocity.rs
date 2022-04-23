use crate::entity::component::pos::PositionComp;
use mlua::{FromLua, Lua, Value};
use rustaria_util::math::{Vector2D, WorldSpace};
use rustaria_util::ty::Pos;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct VelocityComp {
	pub velocity: Vector2D<f32, WorldSpace>,
}

impl VelocityComp {
	pub fn tick(&mut self, pos: &mut PositionComp) {
		pos.position += self.velocity;
	}
}

impl FromLua for VelocityComp {
	fn from_lua(lua_value: Value, lua: &Lua) -> mlua::Result<Self> {
		Ok(VelocityComp {
			velocity: Pos::from_lua(lua_value, lua)?.into(),
		})
	}
}
