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

use mlua::{FromLua, Lua, LuaSerdeExt};
use mlua::prelude::{LuaResult, LuaUserData, LuaUserDataMethods};

use rustaria_util::blake3::Hasher;
use rustaria_util::debug;

use crate::{Prototype, RawId};
use crate::tag::Tag;

/// A registry containing and managing user-added data to Rustaria.
/// See the [module documentation](index.html) for more details.
#[derive(Debug, Clone)]
pub struct Registry<P: Prototype> {
    tag_to_id: HashMap<Tag, RawId>,
    id_to_tag: Vec<Tag>,
    entries: Vec<P>,
}

impl<P: Prototype> Registry<P> {
    pub fn entries(&self) -> &[P] {
        &self.entries
    }
    pub fn get_tag_from_id(&self, id: RawId) -> Option<&Tag> {
        self.id_to_tag.get(id as usize)
    }
    pub fn get_id_from_tag(&self, tag: &Tag) -> Option<RawId> {
        self.tag_to_id.get(tag).copied()
    }
    pub fn get_from_id(&self, id: RawId) -> Option<&P> {
        self.entries.get(id as usize)
    }
    pub fn get_from_id_mut(&mut self, id: RawId) -> Option<&mut P> {
        self.entries.get_mut(id as usize)
    }
    pub fn get_from_tag(&self, tag: &Tag) -> Option<&P> {
        self.get_from_id(self.get_id_from_tag(tag)?)
    }

    pub fn create_from_tag(&self, tag: &Tag) -> Option<P::Item> {
        let id = self.get_id_from_tag(tag)?;
        let prototype = self.get_from_id(id)?;
        Some(prototype.create(id))
    }

    pub fn clear(&mut self) {
        self.tag_to_id.clear();
        self.id_to_tag.clear();
        self.entries.clear();
    }
}

#[derive(Clone)]
pub struct RegistryBuilder<P: Prototype> {
    name: &'static str,
    entries: HashMap<Tag, P>,
}

impl<P: Prototype> RegistryBuilder<P> {
    pub fn new(name: &'static str) -> RegistryBuilder<P> {
        RegistryBuilder {
            name,
            entries: Default::default(),
        }
    }

    pub fn register(self, lua: &Lua) -> LuaResult<()> {
        lua.globals().set(self.name, self)
    }

    pub fn combine(&mut self, other: Self) {
        self.entries.extend(other.entries);
    }

    pub fn finish(self, hasher: &mut Hasher) -> Registry<P> {
        let mut data: Vec<(Tag, P)> = self.entries.into_iter().collect();
        data.sort_by(|(i1, _), (i2, _)| i1.to_string().cmp(&i2.to_string()));

        for (id, (tag, _)) in data.iter().enumerate() {
            hasher.update(&id.to_be_bytes());
            hasher.update(tag.to_string().as_bytes());
        }

        let mut tag_to_id = HashMap::new();
        let mut id_to_tag = Vec::new();
        let mut entries = Vec::new();

        for (id, (tag, prototype)) in data.into_iter().enumerate() {
            tag_to_id.insert(tag.clone(), id as RawId);
            id_to_tag.push(tag);
            entries.push(prototype);
        }

        Registry {
            tag_to_id,
            id_to_tag,
            entries,
        }
    }
}
impl<P: Prototype> LuaUserData for RegistryBuilder<P> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(m: &mut M)
    where
        P: FromLua<'lua>,
    {
        m.add_method_mut("register", |_lua, this, t: HashMap<Tag, P>| {
            debug!(target: this.name, "Registered entries to registry");
            for k in t.keys() {
                debug!("{k}")
            }
            this.entries.extend(t);
            Ok(())
        });
        m.add_method("default", |lua, _this, t| lua.from_value::<P>(t));
    }
}
