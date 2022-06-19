use std::{any::Any, marker::PhantomData};

use eyre::{Context, Report};
use fxhash::FxHashMap;
use eyre::Result;
use apollo::{Lua, Table, Value};
use apollo::impl_macro::*;

use crate::{
	api::{luna::table::LunaTable, prototype::Prototype, registry::Registry, util::lua_table},
	ty::identifier::Identifier,
};

const DEFAULT_PRIORITY: f32 = 69420.0;

pub struct RegistryBuilder<P: Prototype> {
	tables: Vec<Table>,
	_p: PhantomData<P>,
}

impl<P: Prototype> RegistryBuilder<P> {
	pub fn new() -> RegistryBuilder<P> {
		RegistryBuilder {
			tables: vec![],
			_p: Default::default(),
		}
	}

	pub fn register(&mut self, lua: &Lua, value: Table) -> Result<()> {
		self.tables.push(value);
		Ok(())
	}

	pub fn build(&mut self, lua: &Lua) -> Result<Registry<P>> {
		let mut values = FxHashMap::default();
		for table in &self.tables {
			for value in table.clone().iter::<Value, Value>() {
				let (key, value) = value?;
				let (identifier, priority) = Self::get_prototype_entry_key(key)
					.wrap_err("Failed to get registry entry key.")?;
				let prototype = Self::get_prototype(lua, value)
					.wrap_err_with(|| format!("Failed to create prototype {}", identifier))?;
				values.insert(identifier, (priority, prototype));
			}
		}
		Ok(Registry::new(values))
	}

	fn get_prototype(lua: &Lua, value: Value) -> eyre::Result<P> {
		let table = lua_table(value)?;
		let prototype = P::from_lua(LunaTable { lua, table })?;
		Ok(prototype)
	}

	fn get_prototype_entry_key( key: Value) -> eyre::Result<(Identifier, f32)> {
		Ok(match key {
			val @ Value::String(_) => (Identifier::new_lua(val)?, DEFAULT_PRIORITY),
			Value::Table(table) => (
				Identifier::new_lua(table.get::<_, Value>("name")?)?,
				table
					.get::<_, Option<f32>>("priority")?
					.unwrap_or(DEFAULT_PRIORITY),
			),
			_ => {
				return Err(Report::msg(
					"Registry type must be Table { name = , priority = } or an identifier",
				))
			}
		})
	}
}

pub trait DynRegistryBuilder: 'static + Send {
	fn lua_register(&mut self, lua: &Lua, value: Table) -> Result<()>;
	fn build(&mut self, lua: &Lua) -> Box<dyn Any>;
}

impl<P: Prototype > DynRegistryBuilder for RegistryBuilder<P> {
	fn lua_register(&mut self, lua: &Lua, value: Table) -> Result<()> {
		self.register(lua, value)
	}

	fn build(&mut self, lua: &Lua) -> Box<dyn Any> {
		Box::new(self.build(lua))
	}
}

pub struct LuaRegistryBuilder {
	inner: Box<dyn DynRegistryBuilder>,
}

impl LuaRegistryBuilder {
	pub fn new<P: Prototype>(inner: RegistryBuilder<P>) -> LuaRegistryBuilder {
		LuaRegistryBuilder {
			inner: Box::new(inner),
		}
	}

	pub fn build<P: Prototype>(
		mut self,
		lua: &Lua,
	) -> Result<Registry<P>> {
		*self
			.inner
			.build(lua)
			.downcast::<Result<Registry<P>>>()
			.expect("we fucked up hard with downcasting here")
	}
}

#[lua_impl]
impl LuaRegistryBuilder {
	#[lua_method(register)]
	pub fn lua_register(&mut self, lua: &Lua, value: Table) -> Result<()> {
		self.inner.lua_register(lua, value)
	}
}
