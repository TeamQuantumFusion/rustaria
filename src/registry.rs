//! Registries containing and managing user-added data to Rustaria.
//!
//! Each element has an associated [*tag*](Tag) and [*raw ID*](RawId), which could be used
//! in retrieving the element.
//!
//! Tags are textual, human-readable strings that are separated into two
//! parts by a colon (`:`), like `rustaria-core:air` or `your-plugin:custom_thing`.
//! They identify and locate elements in a registry, persistently.
//!
//! Raw IDs, on the other hand, are integers that, like tags, identify and locate elements,
//! but non-persistent and registry-specific. As a part of that tradeoff, they are far
//! cheaper to copy and send around.
//! See [this section](#when-should-i-use-raw-ids-over-tagsvice-versa) for more information.
//!
//! # Example
//! ```
//! # use rustaria::registry::Registry;
//! # fn main() -> eyre::Result<()> {
//! // Everyone likes dumplings. Right?
//! struct Dumpling {
//!     is_pot_sticker: bool,
//! }
//!
//! let derpling = Dumpling {
//!     is_pot_sticker: false,
//! };
//! let cool_dumpling = Dumpling {
//!     is_pot_sticker: true,
//! };
//!
//! // Registering
//! let mut registry = Registry::new("dumplings");
//! registry.register("example:derpling".parse()?, derpling);
//! let cool_dumpling_id = registry.register("example:cool_dumpling".parse()?, cool_dumpling);
//!
//! // Retrieving
//! // You can get something via its tag: (recommended)
//! let derpling = registry.get_from_tag(&"example:derpling".parse()?);
//! assert_eq!(derpling.unwrap().is_pot_sticker, false);
//!
//! // Or by its raw ID: (CAUTION - read on)
//! let cool_dumpling = registry.get_from_id(cool_dumpling_id);
//! assert_eq!(cool_dumpling.unwrap().is_pot_sticker, true);
//! # Ok(())
//! # }
//! ```
//!
//! # When should I use raw IDs over tags/vice versa?
//! In general, raw IDs are **not** safe as long-term pointers to an element.
//! This is because raw IDs are not persistent – exact arrangement and layouts that
//! they are tied to are subject to change, potentially allowing for more convenient
//! or efficient data storage and optimizations.
//!
//! Additionally, raw IDs should not be used across registries, as they are bound
//! to the exact registry in which they are created. Using them elsewhere will
//! most likely _not_ yield desired results.
//!
//! Tags, on the other hand, _are_ persistent, and they are recommended for use
//! in save files, data files, etc, as the tag will always be the same if the
//! registry stayed persistent, regardless of its inner layout details.
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fmt::Display, str::FromStr};

use mlua::prelude::*;
use serde::{Deserialize, Deserializer};
use thiserror::Error;
use tracing::debug;

use crate::blake3::Hasher;

/// A registry containing and managing user-added data to Rustaria.
/// See the [module documentation](index.html) for more details.
#[derive(Debug, Clone)]
pub struct Registry<P> {
    pub(crate) tag_to_id: HashMap<Tag, RawId>,
    pub(crate) id_to_tag: Vec<Tag>,
    pub(crate) entries: Vec<P>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            tag_to_id: HashMap::new(),
            id_to_tag: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[T] {
        &self.entries
    }
    pub fn get_tag_from_id(&self, id: RawId) -> Option<&Tag> {
        self.id_to_tag.get(id as usize)
    }
    pub fn get_id_from_tag(&self, tag: &Tag) -> Option<RawId> {
        self.tag_to_id.get(tag).copied()
    }
    pub fn get_from_id(&self, id: RawId) -> Option<&T> {
        self.entries.get(id as usize)
    }
    pub fn get_from_id_mut(&mut self, id: RawId) -> Option<&mut T> {
        self.entries.get_mut(id as usize)
    }
    pub fn get_from_tag(&self, tag: &Tag) -> Option<&T> {
        self.get_from_id(self.get_id_from_tag(tag)?)
    }
    pub fn get_from_tag_mut(&mut self, tag: &Tag) -> Option<&mut T> {
        self.get_from_id_mut(self.get_id_from_tag(tag)?)
    }
    pub(crate) fn clear(&mut self) {
        self.tag_to_id.clear();
        self.id_to_tag.clear();
        self.entries.clear();
    }
}
impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

// This is lua input (or rust) that gets converted to id,
// by the registry map.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tag {
    pub plugin_id: String,
    pub name: String,
}

impl FromStr for Tag {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((plugin_id, name)) => Ok(Self {
                plugin_id: plugin_id.into(),
                name: name.into(),
            }),
            None => Err(ParseTagError::NotColonSeparated),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Tag { plugin_id, name } = self;
        write!(f, "{plugin_id}:{name}")
    }
}

impl LuaUserData for Tag {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(f: &mut F) {
        f.add_field_method_get("plugin_id", |_, t| Ok(t.plugin_id.clone()));
        f.add_field_method_get("name", |_, t| Ok(t.name.clone()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{this}"))
        });
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ParseTagError {
    #[error("Tag is not encoded as a colon-separated string")]
    NotColonSeparated,
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;

        struct TagVisitor;
        impl<'de> de::Visitor<'de> for TagVisitor {
            type Value = Tag;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a colon-separated string representing a registry tag")
            }
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Tag::from_str(v).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(TagVisitor)
    }
}

// kernel identification
pub type RawId = u32;

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageKey {
    // TODO
}

pub struct RegistryBuilder<T> {
    name: &'static str,
    data: Vec<(Tag, T)>,
}

impl<T> RegistryBuilder<T> {
    pub fn new(name: &'static str) -> RegistryBuilder<T> {
        Self { name, data: vec![] }
    }

    pub fn register(mut self, tag: Tag, element: T) -> Self {
        debug!("Registered '{tag}' registry '{}'", self.name);
        self.data.push((tag, element));
        self
    }

    pub fn register_all(mut self, map: HashMap<Tag, T>) -> Self
    where
        T: Debug
    {
        debug!("Registering all to registry '{}':", self.name);
        for (tag, _) in &map {
            debug!("{tag}");
        }
        debug!("");
        self.data.extend(map);
        self
    }

    pub fn build(mut self, hasher: &mut Hasher) -> Registry<T> {
        self.data
            .sort_by(|(i1, _), (i2, _)| i1.to_string().cmp(&i2.to_string()));

        for (id, (tag, _)) in self.data.iter().enumerate() {
            hasher.update(&id.to_be_bytes());
            hasher.update(tag.to_string().as_bytes());
        }

        let mut registry = Registry::new();

        for (id, (tag, item)) in self.data.into_iter().enumerate() {
            registry.entries.push(item);
            registry.id_to_tag.push(tag.clone());
            registry.tag_to_id.insert(tag, id as u32);
        }

        registry
    }
}
