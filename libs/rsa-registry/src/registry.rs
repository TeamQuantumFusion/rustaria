pub mod builder;
pub mod key;
pub mod lookup;

use std::{
	any::type_name, cmp::Ordering, fmt::Debug, marker::PhantomData, ops::Deref, vec::IntoIter,
};
use std::mem::replace;

use ahash::AHashMap;
use anyways::audit::Audit;
use apollo::{macros::*, FromLua, Lua, UserData, Value};
use log::trace;
use rsa_hash::Hasher;

use crate::{
	id::Id,
	identifier::Identifier,
	registry::lookup::RegistryLookup,
	storage::{Storage, StorageIter},
};

/// A Registry holds a Table of conversion between Identifier and Ids.
#[derive(Clone, Debug)]
pub struct Registry<V> {
	table: Storage<V, V>,
	lookup: RegistryLookup<V>,
}

impl<V> Registry<V> {
	pub fn new(values: AHashMap<Identifier, (f32, V)>) -> Registry<V>
	where
		V: Debug,
	{
		let mut values: Vec<((Identifier, f32), V)> = values
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
			.map(|(id, ((identifier, _), value))| {
				trace!(target: "registry", "Registered {} \"{}\" {value:?}", type_name::<V>().split("::").last().expect("what"), identifier);

				(Id::<V> {
					key: id as u32,
					_v: Default::default()
				}, identifier, value)
			})
			.collect()
	}

	pub fn map<O>(
		self,
		func: impl Fn(Id<O>, &RegistryLookup<O>, V) -> anyways::Result<O>,
	) -> anyways::Result<Registry<O>> {
		let lookup = self.lookup.map::<O>();
		Ok(Registry {
			table: self
				.table
				.into_iter()
				.map(|(id, value)| {
					let id = id.cast();
					Ok((id, func(id, &lookup, value)?))
				})
				.collect::<anyways::Result<Storage<O, O>>>()?,
			lookup,
		})
	}

	/// Takes the source registry and returns a storage with the source registry values but
	/// ids that are mapped from the current registry.
	pub fn zip<S, O>(
		&self,
		source: Registry<S>,
		func: impl Fn(Id<O>,Id<V>, &RegistryLookup<O>, &V, S) -> anyways::Result<O>,
	) -> anyways::Result<Storage<Option<O>, V>> {

		let mut output = Vec::new();
		for (id, _) in self.iter() {
			output.push((id, None));
		}

		let iter = source.table.values.into_iter();
		let lookup: RegistryLookup<O> = source.lookup.map();
		for (id, value) in StorageIter::<S, S, IntoIter<S>>::new(iter) {
			let identifier = lookup.get_identifier(id.cast());
			if let Some(self_id) = self.lookup.get_id(identifier) {
				let _ = replace(
					&mut output[self_id.index()],
					(self_id, Some(func(id.cast(), self_id, &lookup, &self[self_id], value)?)),
				);
			}
		}

		Ok(output.into_iter().collect())
	}

	pub fn lookup(&self) -> &RegistryLookup<V> { &self.lookup }
}

impl<V> Default for Registry<V> {
	fn default() -> Self {
		Registry {
			table: Storage::default(),
			lookup: RegistryLookup::default(),
		}
	}
}
impl<V> Deref for Registry<V> {
	type Target = Storage<V, V>;

	fn deref(&self) -> &Self::Target { &self.table }
}

impl<V> FromIterator<(Id<V>, Identifier, V)> for Registry<V> {
	fn from_iter<T: IntoIterator<Item = (Id<V>, Identifier, V)>>(iter: T) -> Self {
		let mut lookup = Vec::new();
		let mut ident_to_id = AHashMap::default();
		let mut id_to_ident = Vec::new();

		for (id, ident, value) in iter {
			ident_to_id.insert(ident.clone(), id);
			lookup.push((id, value));
			id_to_ident.push((id, ident));
		}

		Registry {
			table: lookup.into_iter().collect(),
			lookup: RegistryLookup {
				id_to_identifier: id_to_ident.into_iter().collect(),
				identifier_to_id: ident_to_id.into_iter().collect(),
			},
		}
	}
}
