use eyre::Result;
use rustaria_util::blake3::Hasher;
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

#[derive(Clone)]
pub struct RegistryBuilder<P: Prototype> {
	entries: HashMap<Tag, P>,
}

impl<P: Prototype> RegistryBuilder<P> {
	pub fn new() -> RegistryBuilder<P> {
		RegistryBuilder {
			entries: HashMap::new()
		}
	}
	
	pub fn register(&mut self, tag: Tag, prototype: P) {
		self.entries.insert(tag, prototype);
	}

	pub fn finish(self, hasher: &mut Hasher) -> Result<Registry<P>> {
		let mut data: Vec<_> = self.entries.into_iter().collect();

		data.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

		for (id, (tag, _)) in data.iter().enumerate() {
			hasher.update(&id.to_be_bytes());
			hasher.update(tag.as_bytes());
		}

		let mut tag_to_id = HashMap::new();
		let mut id_to_tag = Vec::new();
		let mut entries = Vec::new();

		for (id, (tag, prototype)) in data.into_iter().enumerate() {
			tag_to_id.insert(tag.clone(), RawId(id as u32));
			id_to_tag.push(tag);
			entries.push(prototype);
		}

		Ok(Registry {
			tag_to_id,
			id_to_tag,
			entries,
		})
	}
}
