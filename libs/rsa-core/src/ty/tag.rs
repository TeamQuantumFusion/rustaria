use std::fmt::Display;

use mlua::{Error, FromLua, Lua, ToLua, Value};

use crate::api::lua::get_meta;

#[derive(
	Clone,
	Hash,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Debug,
	Default,
	serde::Serialize,
	serde::Deserialize,
)]
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
				let mut meta = get_meta(lua).plugin_id;
				meta.push(':');
				meta.push_str(&tag);
				Self::new(meta)
			}
			Err(err) => Err(err),
		}
	}

	pub fn new<S: Into<String>>(tag: S) -> Result<Tag, TagCreationError> {
		let tag = tag.into();
		if &tag[0..2] == "r:" {
			return Ok(Tag {
				inner: "rustaria:".to_owned() + &tag[2..],
				colon_index: 8,
			});
		}

		let colon_index = tag.find(':').ok_or(TagCreationError::ColonMissing)?;
		Self::new_internal(tag, colon_index)
	}

	pub fn rsa(tag: &'static str) -> Tag {
		Tag {
			inner: "rustaria:".to_owned() + tag,
			colon_index: 8,
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
	fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
		match value {
			Value::String(string) => Tag::new_lua(string.to_str()?.to_string(), lua)
				.map_err(|err| Error::RuntimeError(err.to_string())),
			_ => Err(Error::SerializeError(format!("{value:?}"))),
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

#[cfg(test)]
mod tests {
	use crate::ty::Tag;

	#[test]
	pub fn basic_tag() {
		assert_eq!(
			Tag::new("mod_id:path").unwrap(),
			Tag {
				inner: "mod_id:path".to_string(),
				colon_index: 6
			}
		)
	}

	#[test]
	pub fn shorthand_tag() {
		assert_eq!(
			Tag::new("r:path").unwrap(),
			Tag {
				inner: "rustaria:path".to_string(),
				colon_index: 8
			}
		)
	}

	#[test]
	pub fn internal_tag() {
		assert_eq!(
			Tag::rsa("path"),
			Tag {
				inner: "rustaria:path".to_string(),
				colon_index: 8
			}
		)
	}
}
