use std::{
	marker::PhantomData,
	slice::{Iter, IterMut},
	vec::IntoIter,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use apollo::{FromLua, ToLua, UserData};
use apollo::macros::*;
use crate::ty::id::Id;
use crate::ty::identifier::Identifier;

#[derive(Debug)]
pub struct IdTable<I, V> {
	values: Vec<V>,
	_i: PhantomData<I>,
}

impl<I, V> IdTable<I, V> {
	pub fn get(&self, id: Id<I>) -> &V { &self.values[id.index()] }

	pub fn get_mut(&mut self, id: Id<I>) -> &mut V {
		&mut self.values[id.index()]
	}

	pub fn iter(&self) -> IdTableIter<I, &V, Iter<V>> {
		IdTableIter::new(self.values.iter())
	}

	pub fn iter_mut(&mut self) -> IdTableIter<I, &mut V, IterMut<V>> {
		IdTableIter::new(self.values.iter_mut())
	}
}

#[lua_impl]
impl<I: 'static, V: UserData + Send + 'static> IdTable<I, V> {
	#[lua_method(get)]
	pub fn lua_get(&self, id: Id<I>) -> &V { &self.values[id.index()] }

	#[lua_method(get_mut)]
	pub fn lua_get_mut(&mut self, id: Id<I>) -> &mut V {
		&mut self.values[id.index()]
	}
}

impl<I, V> Default for IdTable<I, V> {
	fn default() -> Self {
		IdTable {
			values: Vec::new(),
			_i: Default::default(),
		}
	}
}

impl<I, V> FromIterator<(Id<I>, V)> for IdTable<I, V> {
	// this breaks the 100% safety but makes things 100x easier to use sooo ehm. yes
	fn from_iter<T: IntoIterator<Item = (Id<I>, V)>>(iter: T) -> Self {
		let mut items = Vec::new();
		for (id, value) in iter {
			items.push((id, value));
		}

		items.sort_by(|(id0, _), (id1, _)| id0.index().cmp(&id1.index()));

		// safety check
		let mut last_id: Option<usize> = None;
		for (id, _) in &items {
			if let Some(last_id) = last_id {
				if id.index() == last_id {
					panic!("Duplicate id");
				} else if id.index() != last_id + 1 {
					panic!("Skipped id");
				}
			} else if id.index() != 0 {
				panic!("id does not start on 0");
			}

			last_id = Some(id.index());
		}

		IdTable {
			values: items.into_iter().map(|(_, value)| value).collect(),
			_i: Default::default(),
		}
	}
}

impl<I, V: Clone> Clone for IdTable<I, V> {
	fn clone(&self) -> Self {
		IdTable {
			values: self.values.clone(),
			_i: Default::default(),
		}
	}
}

impl<I, V: Serialize> Serialize for IdTable<I, V> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.values.serialize(serializer)
	}
}

impl<'de, I, V: Deserialize<'de>> Deserialize<'de> for IdTable<I, V> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Vec::<V>::deserialize(deserializer).map(|lookup| IdTable {
			values: lookup,
			_i: Default::default(),
		})
	}
}

impl<I, V> IntoIterator for IdTable<I, V> {
	type Item = (Id<I>, V);
	type IntoIter = IdTableIter<I, V, IntoIter<V>>;

	fn into_iter(self) -> Self::IntoIter { IdTableIter::new(self.values.into_iter()) }
}

pub struct IdTableIter<I, V, Iter: Iterator<Item = V>> {
	iter: Iter,
	id: usize,
	_p: PhantomData<I>,
}

impl<I, V, Iter: Iterator<Item = V>> IdTableIter<I, V, Iter> {
	fn new(iter: Iter) -> IdTableIter<I, V, Iter> {
		IdTableIter {
			iter,
			id: 0,
			_p: Default::default(),
		}
	}
}

impl<I, V, Iter: Iterator<Item = V>> Iterator for IdTableIter<I, V, Iter> {
	type Item = (Id<I>, V);

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.iter.next().map(|v| (unsafe { Id::new(self.id) }, v));
		self.id += 1;
		out
	}
}
