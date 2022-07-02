use std::{marker::PhantomData};
use std::collections::HashMap;
use std::fmt::Debug;
use anyways::audit::Audit;

use anyways::Result;
use fxhash::FxHashMap;
use apollo::{FromLua, Lua, Value};
use apollo::macros::*;

use crate::{
	api::{registry::Registry},
	ty::identifier::Identifier,
};

const DEFAULT_PRIORITY: f32 = 69420.0;

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
			self.values.insert(key.identifier, (key.priority.unwrap_or(DEFAULT_PRIORITY), value));
		}
	}

	pub fn build(self) -> Result<Registry<P>> {
		Ok(Registry::new(self.values))
	}
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
	priority: Option<f32>
}

impl FromLua for RegistryKey {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		Ok(match lua_value {
			Value::String(_) => RegistryKey {
				identifier: Identifier::from_lua(lua_value, lua)?,
				priority: None
			},
			Value::Table(table) =>  {
				RegistryKey {
					identifier: Identifier::new_lua(table.get("name")?)?,
					priority: table.get::<_, Option<f32>>("priority")?
				}
			},
			_ => {
				return Err(Audit::new(
					"Registry type must be Table { name = , priority = } or an identifier",
				))
			}
		})
	}
}
