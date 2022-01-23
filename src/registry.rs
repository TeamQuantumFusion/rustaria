use std::{str::FromStr, fmt::Display};

use bimap::BiHashMap;
use serde::Deserialize;
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
        debug!(target: "registry", "{}: Registered {:?}", self.name, tag);
        let id = Id(self.entries.len() as u32);
        self.tag_to_id.insert(tag, id);
        self.entries.push(prototype);
        id
    }

    pub fn entries(&self) -> &[P] {
        &self.entries
    }

    pub fn get_id(&self, tag: &Tag) -> Option<&Id> {
        self.tag_to_id.get_by_left(tag)
    }

    pub fn get_tag(&self, id: &Id) -> Option<&Tag> {
        self.tag_to_id.get_by_right(id)
    }

    pub fn get_entry(&self, id: &Id) -> Option<&P> {
        self.entries.get(id.0 as usize)
    }
}

// This is lua input (or rust) that gets converted to id,
// by the registry map.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
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
            None => Err(NotColonSeparated)
        }
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

// kernel identification
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Id(pub u32);

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageKey {
    // TODO
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetLocation(pub String, pub String);
