use std::{
	collections::HashSet,
	fmt::{Debug, Display},
};
use std::hash::Hash;

use mlua::{Error, FromLua, Lua, ToLua, Value};
use serde::{Deserialize, Serialize};

use crate::lua::PluginLua;

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

	pub fn new_lua(tag: String, lua: &Lua) -> Result<Tag, TagCreationError> {
		match Self::new(tag.clone()) {
			Ok(tag) => Ok(tag),
			Err(TagCreationError::ColonMissing) => {
				let mut new_tag = PluginLua::import(lua).id;
				new_tag.push(':');
				new_tag.push_str(&tag);
				Self::new(new_tag)
			}
			Err(err) => Err(err),
		}
	}

	pub fn new<S: Into<String>>(tag: S) -> Result<Tag, TagCreationError> {
		let tag = tag.into();
		let colon_index = tag.find(':').ok_or(TagCreationError::ColonMissing)?;
		Self::new_internal(tag, colon_index)
	}

	pub fn builtin(tag: &str) -> Tag {
		Tag {
			inner: "builtin:".to_owned() + tag,
			colon_index: 7
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

impl FromLua for Tag {
	fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
		match value {
			mlua::Value::String(string) => Tag::new_lua(string.to_str()?.to_string(), lua)
				.map_err(|err| Error::RuntimeError(err.to_string())),
			_ => Err(mlua::Error::SerializeError(format!("{value:?}"))),
		}
	}
}

impl ToLua for Tag {
	fn to_lua(self, lua: &Lua) -> mlua::Result<Value> {
		Ok(Value::String(lua.create_string(&self.inner)?))
	}
}

impl Display for Tag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.inner)
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

pub trait Prototype: Send + Clone + Sync + FromLua + 'static + Debug {
	type Item;

	fn create(&self, id: RawId) -> Self::Item;
	fn get_sprites(&self, _sprites: &mut HashSet<Tag>) {}
	fn lua_registry_name() -> &'static str {
		"nil"
	}
}
