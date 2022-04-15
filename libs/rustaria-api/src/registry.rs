use std::collections::HashMap;
use std::iter::Enumerate;
use std::slice::Iter;

use crate::ty::{Prototype, RawId, Tag};

pub struct Registry<P: Prototype> {
    pub(crate) tag_to_id: HashMap<Tag, RawId>,
    pub(crate) id_to_tag: Vec<Tag>,
    pub(crate) entries: Vec<P>,
}

impl<P: Prototype> Registry<P> {
    pub fn new() -> Registry<P> {
        Registry {
            tag_to_id: HashMap::new(),
            id_to_tag: Vec::new(),
            entries: Vec::new(),
        }
    }

    // Iteration
    pub fn iter(&self) -> Iter<P> {
        self.entries.iter()
    }

    pub fn iter_id(&self) -> Enumerate<Iter<P>> {
        self.entries.iter().enumerate()
    }

    // Get
    pub fn get_id(&self, tag: &Tag) -> Option<RawId> {
        self.tag_to_id.get(tag).copied()
    }

    pub fn get_tag(&self, id: RawId) -> Option<&Tag> {
        self.id_to_tag.get(id.index())
    }

    pub fn get_prototype(&self, id: RawId) -> Option<&P> {
        self.entries.get(id.index())
    }

    //
    pub fn reload(&mut self) {
        self.tag_to_id.clear();
        self.id_to_tag.clear();
        self.entries.clear();
    }
}
