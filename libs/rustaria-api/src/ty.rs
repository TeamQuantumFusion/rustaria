use std::collections::HashMap;
use std::hash::Hash;
use std::{
	collections::HashSet,
	fmt::{Debug, Display},
};

use mlua::{Error, ExternalError, FromLua, Lua, ToLua, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::lua::ctx;

// Raw Ids
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RawId(pub u32);

impl RawId {
	pub fn index(self) -> usize {
		self.0 as usize
	}
}

pub type PluginId = String;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub struct Tag {
	pub(crate) inner: String,
	pub(crate) colon_index: u8,
}

impl Tag {
	fn new_internal(tag: String, colon_index: usize) -> Result<Tag, TagCreationError> {
		if colon_index >= 255 {
			return Err(TagCreationError::CharacterLimit);
		}

		Ok(Tag {
			inner: tag,
			colon_index: colon_index as u8,
		})
	}

	pub fn new<S: Into<String>>(tag: S) -> Result<Tag, TagCreationError> {
		let tag = tag.into();
		let colon_index = tag.find(':').ok_or(TagCreationError::ColonMissing)?;
		Self::new_internal(tag, colon_index)
	}

	pub fn new_lua(tag: String, lua: &Lua) -> Result<Tag, TagCreationError> {
		match Self::new(tag.clone()) {
			Ok(tag) => Ok(tag),
			Err(TagCreationError::ColonMissing) => {
				let mut new_tag = ctx(lua).id;
				new_tag.push(':');
				new_tag.push_str(&tag);
				Self::new(new_tag)
			}
			Err(err) => Err(err),
		}
	}

	pub fn as_bytes(&self) -> &[u8] {
		self.inner.as_bytes()
	}
	pub fn plugin_id(&self) -> &str {
		&self.inner[..self.colon_index as usize]
	}

	pub fn identifier(&self) -> &str {
		&self.inner[self.colon_index as usize + 1..]
	}
}

impl Display for Tag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.inner)
	}
}

impl LuaConvertableCar for Tag {
	fn from_luaagh(value: Value, lua: &Lua) -> mlua::Result<Self> {
		match value {
			mlua::Value::String(string) => {
				Tag::new_lua(string.to_str()?.to_string(), lua).map_err(|err| err.to_lua_err())
			}
			_ => Err(mlua::Error::SerializeError(format!("{value:?}"))),
		}
	}

	fn into_luaagh(self, lua: &Lua) -> mlua::Result<Value> {
		Ok(Value::String(lua.create_string(&self.inner)?))
	}
}

impl FromLua for Tag {
	fn from_lua(lua_value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
		Tag::from_luaagh(lua_value, lua)
	}
}

impl ToLua for Tag {
	fn to_lua(self, lua: &Lua) -> mlua::Result<Value> {
		Tag::into_luaagh(self, lua)
	}
}

#[derive(thiserror::Error, Debug)]
pub enum TagCreationError {
	#[error("Could not find a colon, and plugin context is not available to inherit from.")]
	ColonMissing,
	#[error("Reached colon limit (255)")]
	CharacterLimit,
	#[error("Illegal characters were found")]
	IllegalCharacters,
}

pub trait Prototype: Clone + Send + Sync + 'static + Debug + DeserializeOwned + Serialize {
	type Item;

	fn create(&self, id: RawId) -> Self::Item;
	fn get_sprites(&self, _sprites: &mut HashSet<Tag>) {}
	fn lua_registry_name() -> &'static str {
		"nil"
	}
}

// cringe shit because of lua
pub trait LuaConvertableCar: Sized {
	fn from_luaagh(value: mlua::Value, lua: &Lua) -> mlua::Result<Self>;
	fn into_luaagh(self, lua: &Lua) -> mlua::Result<mlua::Value>;
}

// A macro for reusing existing lua implementations provided with mlua.
// Named by me, inspired by me not remaking every type implementation.
macro_rules! lazy_alpha {
    ($($TY:ty),*) => {
        $(
        impl LuaConvertableCar for $TY {
            fn from_luaagh(value: Value, lua: &Lua) -> mlua::Result<Self> {
                <$TY>::from_lua(value, lua)
            }

            fn into_luaagh(self, lua: &Lua) -> mlua::Result<Value> {
                self.to_lua(lua)
            }
        }
        )*

    };
}

lazy_alpha!(u8, i8, u16, i16, i32, u32, f32, f64, i64, u64, i128, u128, String);

pub struct LuaCar<T>(pub T);

impl<A: LuaConvertableCar> LuaConvertableCar for Option<A> {
	fn from_luaagh(value: Value, lua: &Lua) -> mlua::Result<Self> {
		match value {
			Value::Nil => Ok(None),
			_ => Ok(Some(A::from_luaagh(value, lua)?)),
		}
	}

	fn into_luaagh(self, lua: &Lua) -> mlua::Result<Value> {
		match self {
			None => Ok(Value::Nil),
			Some(value) => value.into_luaagh(lua),
		}
	}
}

impl<K: LuaConvertableCar + Eq + Hash, V: LuaConvertableCar> LuaConvertableCar for HashMap<K, V> {
	fn from_luaagh(value: Value, lua: &Lua) -> mlua::Result<Self> {
		match value {
			Value::Table(table) => {
				let mut out = HashMap::new();
				for (key, value) in table.pairs::<Value, Value>().flatten() {
					out.insert(K::from_luaagh(key, lua)?, V::from_luaagh(value, lua)?);
				}

				Ok(out)
			}
			_ => Err(Error::DeserializeError("Invalid type".to_string())),
		}
	}

	fn into_luaagh(self, lua: &Lua) -> mlua::Result<Value> {
		let table = lua.create_table()?;

		for (key, value) in self {
			table.set(key.into_luaagh(lua)?, value.into_luaagh(lua)?)?;
		}

		Ok(Value::Table(table))
	}
}

impl<A: LuaConvertableCar> FromLua for LuaCar<A> {
	fn from_lua(lua_value: Value, lua: &Lua) -> mlua::Result<Self> {
		A::from_luaagh(lua_value, lua).map(LuaCar)
	}
}

impl<A: LuaConvertableCar> ToLua for LuaCar<A> {
	fn to_lua(self, lua: &Lua) -> mlua::Result<Value> {
		self.0.into_luaagh(lua)
	}
}
