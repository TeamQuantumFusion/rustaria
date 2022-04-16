use std::{
	collections::HashSet,
	fmt::{Debug, Display},
};

use mlua::{ExternalError, FromLua, Lua, ToLua, Value};
use rustaria_util::info;
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

impl<'lua> FromLua<'lua> for Tag {
	fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		info!("Dog shit");
		match lua_value {
			mlua::Value::String(string) => {
				Tag::new_lua(string.to_str()?.to_string(), lua).map_err(|err| err.to_lua_err())
			}
			_ => Err(mlua::Error::SerializeError(format!("{lua_value:?}"))),
		}
	}
}

impl<'lua> ToLua<'lua> for Tag {
	fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
		Ok(Value::String(lua.create_string(&self.inner)?))
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
		"null"
	}
}

// cringe shit because of lua
pub trait LuaConvertableCar: Sized {
	fn from_luaagh(value: mlua::Value, lua: &Lua) -> mlua::Result<Self>;
	fn into_luaagh(self, lua: &Lua) -> mlua::Result<mlua::Value>;
}

pub struct LuaCar<T>(pub T);

impl<'lua, A: LuaConvertableCar> FromLua<'lua> for LuaCar<A> {
	fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		A::from_luaagh(lua_value, lua).map(LuaCar)
	}
}

impl<'lua, A: LuaConvertableCar> ToLua<'lua> for LuaCar<A> {
	fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
		self.0.into_luaagh(lua)
	}
}
