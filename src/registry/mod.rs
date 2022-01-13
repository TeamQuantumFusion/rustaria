#![allow(unused)] // alpha, remove this when you're done - leocth

use bimap::BiHashMap;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::debug;

pub struct Registry<P> {
    tag_to_id: BiHashMap<Tag, Id>,
    entries: Vec<P>,
    current_id: u32,
}

impl<P> Default for Registry<P> {
    fn default() -> Self {
        Registry::new()
    }
}

impl<P> Registry<P> {
    pub fn new() -> Self {
        Self {
            tag_to_id: Default::default(),
            entries: Default::default(),
            current_id: 0,
        }
    }

    pub fn register(&mut self, tag: Tag, prototype: P) -> Id {
        debug!("Registered {:?}", tag);
        let id = Id(self.current_id);
        self.tag_to_id.insert(tag, id);
        self.entries.insert(self.current_id as usize, prototype);
        self.current_id += 1;
        id
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
pub struct Tag(String);

impl Tag {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}

// kernel identification
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Id(u32);

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageKey {
    // TODO
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetLocation {
    // TODO
}
