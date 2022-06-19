use std::any::type_name;
use std::cmp::Ordering;

use fxhash::{FxBuildHasher, FxHashMap};
use apollo::{ToLua, UserData};
use tracing::trace;

use crate::{
	api::id_table::IdTable,
	ty::{id::Id, identifier::Identifier},
};
use crate::util::blake3::Hasher;
use apollo::impl_macro::*;

pub struct Registry<I> {
	pub table: IdTable<I, I>,
	pub id_to_ident: IdTable<I, Identifier>,
	pub ident_to_id: FxHashMap<Identifier, Id<I>>,
}

impl<I> Registry<I> {
	pub fn get(&self, id: Id<I>) -> &I { self.table.get(id) }

	pub fn get_mut(&mut self, id: Id<I>) -> &mut I { self.table.get_mut(id) }

	pub fn get_identifier(&self, id: Id<I>) -> &Identifier { self.id_to_ident.get(id) }

	pub fn get_id(&self, id: &Identifier) -> Option<Id<I>> { self.ident_to_id.get(id).copied() }

	pub fn into_entries(self) -> impl Iterator<Item = (Id<I>, Identifier, I)> {
		self.table
			.into_iter()
			.zip(self.id_to_ident.into_iter())
			.map(|((id, prototype), (_, identifier))| (id, identifier, prototype))
	}

	pub fn entries(&self) -> impl Iterator<Item = (Id<I>, &Identifier, &I)> {
		self.table
			.iter()
			.zip(self.id_to_ident.iter())
			.map(|((id, prototype), (_, identifier))| (id, identifier, prototype))
	}

	pub fn new(
		values: FxHashMap<Identifier, (f32, I)>,
	) -> Registry<I> {
		let mut values: Vec<((Identifier, f32), I)> = values
			.into_iter()
			.map(|(identifier, (priority, prototype))| ((identifier, priority), prototype))
			.collect();

		values.sort_by(|((id0, priority0), _), ((id1, priority1), _)| {
			let ordering = priority0.total_cmp(priority1);
			if ordering == Ordering::Equal {
				return id0.cmp(id1);
			}
			ordering
		});

		values
			.into_iter()
			.enumerate()
			.map(|(id, ((identifier, _), value))| unsafe {
				trace!(target: "registry", "Registered {} \"{}\"", type_name::<I>().split("::").last().expect("what"), identifier);

				(Id::<I>::new(id), identifier, value)
			})
			.collect()
	}

	pub fn append_hasher(&self, hasher: &mut Hasher) {
		for (id, identifier, _) in self.entries() {
			hasher.update(&id.id().to_be_bytes());
			hasher.update(identifier.path.as_bytes());
			hasher.update(identifier.namespace.as_bytes());
		}
	}
}

#[lua_impl]
impl<I: 'static + UserData + ToLua + Send> Registry<I> {
	#[lua_method(get)]
	pub fn lua_get(&self, id: Id<I>) -> &I {
		self.get(id)
	}

	#[lua_method(get_id)]
	pub fn lua_get_id(&self, id: Identifier) -> Option<Id<I>> {
		self.get_id(&id)
	}

	#[lua_method(get_identifier)]
	pub fn lua_get_identifier(&self, id: Id<I>) -> &Identifier {
		self.get_identifier(id)
	}
}

impl<I> FromIterator<(Id<I>, Identifier, I)> for Registry<I> {
	fn from_iter<T: IntoIterator<Item = (Id<I>, Identifier, I)>>(iter: T) -> Self {
		let mut lookup = Vec::new();
		let mut ident_to_id = FxHashMap::with_hasher(FxBuildHasher::default());
		let mut id_to_ident = Vec::new();

		for (id, ident, value) in iter {
			ident_to_id.insert(ident.clone(), id);
			lookup.push((id, value));
			id_to_ident.push((id, ident));
		}

		Registry {
			table: lookup.into_iter().collect(),
			id_to_ident: id_to_ident.into_iter().collect(),
			ident_to_id,
		}
	}
}

impl<I> Default for Registry<I> {
	fn default() -> Self {
		Registry {
			table: IdTable::default(),
			id_to_ident: IdTable::default(),
			ident_to_id: Default::default(),
		}
	}
}
