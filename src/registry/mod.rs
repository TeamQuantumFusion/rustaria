#![allow(unused)] // alpha, remove this when you're done - leocth

use bimap::BiHashMap;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Registry {
    tag_to_id: BiHashMap<Tag, Id>,
    current_id: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            tag_to_id: Default::default(),
            current_id: 0,
        }
    }

    pub fn register(&mut self, tag: Tag) -> Id {
        let id = Id(self.current_id);
        self.tag_to_id.insert(tag, id);
        id
    }

    pub fn get_id(&self, tag: &Tag) -> Option<&Id> {
        self.tag_to_id.get_by_left(tag)
    }

    pub fn get_tag(&self, id: &Id) -> Option<&Tag> {
        self.tag_to_id.get_by_right(id)
    }
}

// This is lua input (or rust) that gets converted to id,
// by the registry map.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct Tag {
    tag: String,
    category: TagCategory,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagCategory {
    Item,
    Tile,
    Wall,
    Tree,
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
