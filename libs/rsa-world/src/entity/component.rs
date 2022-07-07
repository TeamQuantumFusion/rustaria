use apollo::{FromLua, Function, Lua, LuaSerdeExt, Value};
use rsa_core::api::util::lua_table;
use rsa_core::math::{Rect, Vector2D};
use rsa_core::ty::{DirMap, Id, WS};
use rsa_core::err::Result;
use crate::EntityDesc;
/// Our lovely components
#[macro_export]
macro_rules! iter_components {
	($BLOCK:block) => {{
		type T = $crate::entity::component::PositionComponent;
		$BLOCK;
	}
	{
		type T = $crate::entity::component::PhysicsComponent;
		$BLOCK;
	}
	{
		type T = $crate::entity::component::CollisionComponent;
		$BLOCK;
	}
	{
		type T = $crate::entity::component::HumanoidComponent;
		$BLOCK;
	}
	{
		type T = $crate::entity::component::PrototypeComponent;
		$BLOCK;
	}
	{
		type T = $crate::entity::component::GravityComponent;
		$BLOCK;
	}};
}

#[derive(Clone)]
pub struct PrototypeComponent {
	pub id: Id<EntityDesc>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GravityComponent {
	pub amount: f32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PhysicsComponent {
	pub vel: Vector2D<f32, WS>,
	pub accel: Vector2D<f32, WS>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(transparent)]
pub struct PositionComponent {
	pub pos: Vector2D<f32, WS>,
}

#[derive(Debug, Clone)]
pub struct CollisionComponent {
	pub collision_box: Rect<f32, WS>,
	pub hit_callback: Option<Function>,
	// not serialized
	pub collided: DirMap<bool>,
	pub collisions: Vec<(Rect<f32, WS>, f32)>,
}

impl FromLua for CollisionComponent {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		let table = lua_table(lua_value)?;
		Ok(CollisionComponent {
			collision_box: lua.from_value(table.get("collision_box")?)?,
			hit_callback: table.get("hit_callback")?,
			collided: Default::default(),
			collisions: vec![],
		})
	}
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct HumanoidComponent {
	// Settings
	pub jump_amount: f32,
	pub jump_speed: f32,

	pub run_acceleration: f32,
	pub run_slowdown: f32,
	pub run_max_speed: f32,

	// Runtime stuff
	#[serde(skip)]
	pub dir: Vector2D<f32, WS>,
	#[serde(skip)]
	pub jumping: bool,
	#[serde(skip)]
	pub jumped: bool,
	#[serde(skip)]
	pub jump_ticks_remaining: u32,
}
