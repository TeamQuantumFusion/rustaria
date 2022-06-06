use mlua::{FromLua, Lua, Value};
use serde::Deserialize;

use rsa_core::math::{Rect, WorldSpace};
use rsa_core::ty::Rectangle;

#[derive(Clone, Debug, Deserialize)]
pub struct HitboxComp {
	pub hitbox: Rect<f32, WorldSpace>,
	pub touches_ground: bool,
}

impl FromLua for HitboxComp {
	fn from_lua(lua_value: Value, lua: &Lua) -> mlua::Result<Self> {
		Ok(HitboxComp {
			hitbox: Rectangle::from_lua(lua_value, lua)?.into(),
			touches_ground: false,
		})
	}
}
