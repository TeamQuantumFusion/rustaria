use mlua::{FromLua, Lua};
use mlua::prelude::{LuaError, LuaValue};
use rsa_core::math::{Vector2D, WorldSpace};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HumanoidSettings {
	// Jump
	pub jump_frames: u32,
	pub jump_speed: f32,

	// Run
	pub run_acceleration: f32,
	pub run_slowdown: f32,
	pub run_max_speed: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HumanoidComp {
	pub settings: HumanoidSettings,
	pub direction: Vector2D<f32, WorldSpace>,

	// If its currently jumping
	pub jumping: bool,
	pub jump_frames_remaining: u32,
}

impl FromLua for HumanoidComp {
	fn from_lua(lua_value: LuaValue, _: &Lua) -> mlua::Result<Self> {
		if let LuaValue::Table(table) = lua_value {
			Ok(HumanoidComp {
				settings: HumanoidSettings {
					jump_frames: table.get("jump_frames")?,
					jump_speed: table.get("jump_speed")?,
					run_acceleration: table.get("run_acceleration")?,
					run_slowdown: table.get("run_slowdown")?,
					run_max_speed: table.get("run_max_speed")?,
				},
				direction: Default::default(),
				jumping: false,
				jump_frames_remaining: 0,
			})
		} else {
			return Err(LuaError::UserDataTypeMismatch);
		}
	}
}
