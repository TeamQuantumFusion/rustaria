use std::collections::HashMap;

use serde::Deserialize;

pub struct Registry {
    // TODO(leocth): use BiMap from `bimap`
    tag_to_id: HashMap<Tag, Id>,
    // uses vec instead of hashmap to save 1ns of time in our lifetime
    id_to_tag: Vec<Tag>,
    current_id: u32
}

impl Registry {
    pub fn new() -> Self {
        Self {
            tag_to_id: Default::default(),
            id_to_tag: Default::default(),
            current_id: 0
        }
    }

    pub fn register(&mut self, tag: Tag) -> Id {
        let id = Id (self.current_id);
        self.id_to_tag.insert(id.0 as usize, tag.clone());
        self.tag_to_id.insert(tag, id);
        id
    }

    pub fn get_id(&self, tag: &Tag) -> Option<&Id> {
        self.tag_to_id.get(tag)
    }

    pub fn get_tag(&self, id: &Id) -> Option<&Tag> {
        self.id_to_tag.get(id.0 as usize)
    }
}

// This is lua input (or rust) that gets converted to id,
// by the registry map.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct Tag {
    tag: String,
    category: TagCategory
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagCategory {
    Item,
    Tile,
    Wall,
    Tree
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