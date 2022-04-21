use crate::plugin::Manifest;
use mlua::{Error, FromLua, Lua, Result, ToLua, UserData, Value};

pub mod core;
pub mod reload;

pub fn new_lua(manifest: &Manifest) -> Result<Lua> {
	let lua_state = Lua::new();
	lua_state.globals().set(
		"plugin",
		PluginLua {
			id: manifest.id.clone(),
		},
	)?;
	core::register(&lua_state)?;

	Ok(lua_state)
}

#[macro_export]
macro_rules! lua_struct {
	($TYPE:ident => $($FIELD:ident),*) => {
		impl mlua::FromLua for $TYPE {
			fn from_lua(lua_value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
				if let mlua::Value::Table(table) = lua_value {
					Ok($TYPE {
						$(
						$FIELD: table.get(stringify!($FIELD))?,
						)*
					})
				} else {
					Err(mlua::Error::RuntimeError("Invalid type".to_string()))
				}
			}
		}

		impl mlua::ToLua for $TYPE {
			fn to_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
				let table = lua.create_table()?;
				$(
					table.set(stringify!($FIELD), self.$FIELD)?;
				)*
				Ok(mlua::Value::Table(table))
			}
		}
	};
}

pub enum LuaEnum {
	Test,
	AnotherTest,
	InnerTest { stuff: String, more_stuff: u32 },
}

macro_rules! lua_enum {
	($TYPE:ident => $($ITEM:ident$({
		$($FIELD:ident),*
	})?),*) => {
		impl FromLua for $TYPE {
			fn from_lua(lua_value: Value, _: &Lua) -> Result<Self> {
				if let mlua::Value::Table(table) = lua_value {
					let (item, values): (String, Value) = table
						.pairs()
						.flatten()
						.next()
						.ok_or_else(|| mlua::Error::RuntimeError("Invalid enum option".to_string()))?;
					match item.as_str() {
						$(
							stringify!($ITEM) => Ok(
								{
									let _field_table = if let mlua::Value::Table(s) = values {
										Ok(s)
									} else {
										Err(mlua::Error::RuntimeError(format!("Invalid type on enum field.")))
									}?;

									$TYPE::$ITEM $(
										{$(
											$FIELD: {
												_field_table.get(stringify!($FIELD))?
											},
										)*}
									)?
								}
							),
						)*
						_ => Err(mlua::Error::RuntimeError(format!(
							"Unknown enum type {}",
							item
						))),
					}
				} else {
					Err(mlua::Error::RuntimeError("Invalid type".to_string()))
				}
			}
		}
	};
}
lua_enum!(LuaEnum => Test, AnotherTest, InnerTest {stuff, more_stuff});

//impl FromLua for LuaEnum {
//	fn from_lua(lua_value: Value, _: &Lua) -> Result<Self> {
//		if let mlua::Value::Table(table) = lua_value {
//			let (item, values): (String, Value) = table
//				.pairs()
//				.flatten()
//				.next()
//				.ok_or_else(|| mlua::Error::RuntimeError("Invalid enum option".to_string()))?;
//			match item.as_str() {
//				"Test" => Ok(LuaEnum::Test),
//				"AnotherTest" => Ok(LuaEnum::AnotherTest {}),
//				"InnerTest" => {
//					if let mlua::Value::Table(table) = values {
//						Ok(LuaEnum::InnerTest {
//							stuff: table.get("stuff")?,
//							more_stuff: table.get("more_stuff")?,
//						})
//					} else {
//						Err(mlua::Error::RuntimeError("Invalid type".to_string()))
//					}
//				}
//				_ => Err(mlua::Error::RuntimeError(format!(
//					"Unknown enum type {}",
//					item
//				))),
//			}
//		} else {
//			Err(mlua::Error::RuntimeError("Invalid type".to_string()))
//		}
//	}
//}

#[derive(Clone)]
pub struct PluginLua {
	pub id: String,
}

impl UserData for PluginLua {}

impl PluginLua {
	pub fn import(lua: &Lua) -> PluginLua {
		lua.globals()
			.get("plugin")
			.expect("Could not get plugin global.")
	}
}
