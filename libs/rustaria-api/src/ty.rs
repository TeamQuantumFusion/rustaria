use std::hash::Hash;
use std::{
	collections::HashSet,
	fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

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

#[derive(thiserror::Error, Debug)]
pub enum TagCreationError {
	#[error("Could not find a colon, and plugin context is not available to inherit from.")]
	ColonMissing,
	#[error("Reached colon limit (255)")]
	CharacterLimit,
	#[error("Illegal characters were found")]
	IllegalCharacters,
}

pub trait Prototype: Send + Sync + 'static + Debug {
	type Item;

	fn create(&self, id: RawId) -> Self::Item;
	fn get_sprites(&self, _sprites: &mut HashSet<Tag>) {}
	fn lua_registry_name() -> &'static str {
		"nil"
	}
}
