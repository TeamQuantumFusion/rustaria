use std::{any::type_name, cmp::Ordering, fmt::Debug, marker::PhantomData};

use anyways::audit::Audit;
use apollo::{macros::*, FromLua, Lua, UserData, Value};

use crate::{
	blake3::Hasher,
	err::Result,
	log::trace,
	std::{FxBuildHasher, FxHashMap},
	ty::{id_table::IdTable, Id, Identifier},
};

pub const DEFAULT_PRIORITY: f32 = 1000.0;

#[derive(Clone, Debug)]
pub struct Registry<I> {
	pub table: IdTable<I, I>,
	pub id_to_ident: IdTable<I, Identifier>,
	pub ident_to_id: FxHashMap<Identifier, Id<I>>,
}

impl<I> Registry<I> {
	pub fn get(&self, id: Id<I>) -> &I { self.table.get(id) }

	pub fn get_mut(&mut self, id: Id<I>) -> &mut I { self.table.get_mut(id) }

	pub fn get_identifier(&self, id: Id<I>) -> &Identifier { self.id_to_ident.get(id) }

	pub fn get_id_from_identifier(&self, id: &Identifier) -> Option<Id<I>> {
		self.ident_to_id.get(id).copied()
	}

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

	pub fn new(values: FxHashMap<Identifier, (f32, I)>) -> Registry<I>
	where
		I: Debug,
	{
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
				trace!(target: "registry", "Registered {} \"{}\" {value:?}", type_name::<I>().split("::").last().expect("what"), identifier);

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

#[cfg(feature = "testing")]
impl<I> Registry<I> {
	pub fn get_value(&self, identifier: &Identifier) -> Option<&I> {
		Some(self.get(self.get_id_from_identifier(identifier)?))
	}
}

#[lua_impl]
impl<I: UserData + 'static + Send> Registry<I> {
	#[lua_method(get)]
	pub fn lua_get(&self, id: Id<I>) -> &I { self.get(id) }

	#[lua_method(get_id)]
	pub fn lua_get_id(&self, id: Identifier) -> Option<Id<I>> { self.get_id_from_identifier(&id) }

	#[lua_method(get_identifier)]
	pub fn lua_get_identifier(&self, id: Id<I>) -> Identifier { self.get_identifier(id).clone() }
}

impl<I> FromIterator<(Id<I>, Identifier, I)> for Registry<I> {
	fn from_iter<T: IntoIterator<Item = (Id<I>, Identifier, I)>>(iter: T) -> Self {
		let mut lookup = Vec::new();
		let mut ident_to_id = FxHashMap::default();
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

#[derive(Debug)]
pub struct RegistryBuilder<P: FromLua + 'static> {
	pub values: FxHashMap<Identifier, (f32, P)>,
	_p: PhantomData<P>,
}

#[lua_impl]
impl<P: FromLua + 'static + Debug> RegistryBuilder<P> {
	pub fn new() -> RegistryBuilder<P> {
		RegistryBuilder {
			values: FxHashMap::default(),
			_p: Default::default(),
		}
	}

	#[lua_method]
	pub fn register(&mut self, values: Vec<(RegistryKey, P)>) {
		for (key, value) in values {
			self.values.insert(
				key.identifier,
				(key.priority.unwrap_or(DEFAULT_PRIORITY), value),
			);
		}
	}

	pub fn build(self) -> Result<Registry<P>> { Ok(Registry::new(self.values)) }
}

impl<P: FromLua + 'static + Debug> FromLua for RegistryBuilder<P> {
	fn from_lua(value: Value, lua: &Lua) -> Result<RegistryBuilder<P>> {
		let values: Vec<(RegistryKey, P)> = lua.unpack(value)?;
		let mut builder = RegistryBuilder::new();
		builder.register(values);
		Ok(builder)
	}
}

#[derive(Debug)]
pub struct RegistryKey {
	identifier: Identifier,
	priority: Option<f32>,
}

impl FromLua for RegistryKey {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		Ok(match lua_value {
			Value::String(_) => RegistryKey {
				identifier: Identifier::from_lua(lua_value, lua)?,
				priority: None,
			},
			Value::Table(table) => RegistryKey {
				identifier: Identifier::new_lua(table.get("name")?)?,
				priority: table.get::<_, Option<f32>>("priority")?,
			},
			_ => {
				return Err(Audit::new(
					"Registry type must be Table { name = , priority = } or an identifier",
				));
			}
		})
	}
}
