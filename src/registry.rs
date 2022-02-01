use std::{fmt::Display, str::FromStr};

use bimap::BiHashMap;
use serde::{Deserialize, Deserializer};
use tracing::debug;

pub struct Registry<P> {
    name: &'static str,
    tag_to_id: BiHashMap<Tag, Id>,
    entries: Vec<P>,
}

impl<P> Registry<P> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            tag_to_id: Default::default(),
            entries: Default::default(),
        }
    }

    pub fn register(&mut self, tag: Tag, prototype: P) -> Id {
        debug!("Registered {} '{}'", self.name, tag);
        let id = self.entries.len() as Id;
        self.tag_to_id.insert(tag, id);
        self.entries.push(prototype);
        id
    }

    pub fn entries(&self) -> &[P] {
        &self.entries
    }

    pub fn get_tag_from_id(&self, id: Id) -> Option<&Tag> {
        self.tag_to_id.get_by_right(&id)
    }

    pub fn get_id_from_tag(&self, tag: &Tag) -> Option<Id> {
        self.tag_to_id.get_by_left(tag).copied()
    }

    pub fn get_from_id(&self, id: Id) -> Option<&P> {
        self.entries.get(id as usize)
    }

    pub fn get_from_tag(&self, tag: &Tag) -> Option<&P> {
        self.get_from_id(self.get_id_from_tag(tag)?)
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
    type Err = NotColonSeparated;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((plugin_id, name)) => Ok(Self {
                plugin_id: plugin_id.into(),
                name: name.into(),
            }),
            None => Err(NotColonSeparated),
        }
    }
}
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Tag { plugin_id, name } = self;
        write!(f, "{plugin_id}:{name}")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NotColonSeparated;
impl Display for NotColonSeparated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag is not encoded as a colon-separated string")
    }
}
impl std::error::Error for NotColonSeparated {}

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
pub type Id = u32;

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageKey {
    // TODO
}
