use mlua::{FromLua, Lua, Value};
use std::env::current_dir;

use rustaria_api::{
	ty::{LuaConvertableCar, Prototype, Tag},
	Api, Carrier,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TilePrototype {
	eggs: Option<Tag>,
	name: String,
}

impl Prototype for TilePrototype {
	type Item = ();

	fn create(&self, _: rustaria_api::ty::RawId) -> Self::Item {
		todo!()
	}

	fn lua_registry_name() -> &'static str {
		"Tiles"
	}
}

impl FromLua for TilePrototype {
	fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
		if let mlua::Value::Table(table) = value {
			Ok(TilePrototype {
				eggs: table.get("eggs")?,
				name: table.get("name")?,
			})
		} else {
			Err(mlua::Error::UserDataTypeMismatch)
		}
	}
}

#[test]
fn it_adds_two() {
	rustaria_util::initialize().unwrap();

	let mut path = current_dir().unwrap();
	path.push("tests");
	path.push("basic_plugin");

	let mut api = Api::new(path, vec![]).unwrap();
	let mut stack = Carrier::default();

	let mut reload = api.reload(&mut stack);
	reload.add_reload_registry::<TilePrototype>().unwrap();
	reload.reload();
	reload.add_apply_registry::<TilePrototype>().unwrap();
	reload.apply();
}
