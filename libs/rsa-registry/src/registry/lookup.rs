use ahash::AHashMap;
use rsa_hash::Hasher;
use crate::{Id, Identifier, Storage};

#[derive(Clone, Debug)]
pub struct RegistryLookup<V> {
	pub(crate) id_to_identifier: Storage<Identifier, V>,
	pub(crate) identifier_to_id: AHashMap<Identifier, Id<V>>,
}

impl<V> RegistryLookup<V> {

	pub fn get_id(&self, ident: &Identifier) -> Option<Id<V>> {
		self.identifier_to_id.get(ident).copied()
	}

	pub fn get_identifier(&self, id: Id<V>) -> &Identifier {
		&self.id_to_identifier[id]
	}

	pub fn map<O>(self) -> RegistryLookup<O> {
		RegistryLookup {
			id_to_identifier: self
				.id_to_identifier
				.into_iter()
				.map(|(id, ident)| (id.cast(), ident))
				.collect(),
			identifier_to_id:  self
				.identifier_to_id
				.into_iter()
				.map(|(id, ident)| (id, ident.cast()))
				.collect()
		}
	}

	pub fn append_hasher(&self, hasher: &mut Hasher) {
		for (id, identifier) in self.id_to_identifier.values.iter().enumerate() {
			hasher.update(&(id as u32).to_le_bytes());
			hasher.update(identifier.namespace.as_bytes());
			hasher.update(identifier.path.as_bytes());
		}
	}
	pub fn id_to_identifier(&self) -> &Storage<Identifier, V> {
		&self.id_to_identifier
	}
	pub fn identifier_to_id(&self) -> &AHashMap<Identifier, Id<V>> {
		&self.identifier_to_id
	}
}


impl<V> Default for RegistryLookup<V> {
	fn default() -> Self {
		RegistryLookup {
			id_to_identifier: Storage::default(),
			identifier_to_id: Default::default()
		}
	}
}