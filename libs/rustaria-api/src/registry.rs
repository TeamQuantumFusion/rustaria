use std::collections::HashMap;
use std::slice::Iter;

use crate::ty::{Prototype, RawId, Tag};

#[derive(Default)]
pub struct Registry<P: Prototype> {
	pub(crate) tag_to_id: HashMap<Tag, RawId>,
	pub(crate) id_to_tag: Vec<Tag>,
	pub(crate) entries: Vec<P>,
}

impl<P: Prototype> Registry<P> {
	pub fn iter(&self) -> Iter<P> {
		self.entries.iter()
	}

	pub fn id_from_tag(&self, tag: &Tag) -> Option<RawId> {
		self.tag_to_id.get(tag).copied()
	}

	pub fn tag_from_id(&self, id: RawId) -> Option<&Tag> {
		self.id_to_tag.get(id.index())
	}

	pub fn prototype_from_id(&self, id: RawId) -> Option<&P> {
		self.entries.get(id.index())
	}

	pub fn reload(&mut self) {
		self.tag_to_id.clear();
		self.id_to_tag.clear();
		self.entries.clear();
	}
}
