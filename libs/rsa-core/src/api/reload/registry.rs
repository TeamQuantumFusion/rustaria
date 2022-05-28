use std::collections::HashMap;

use mlua::{Lua, Result as LuaResult, Table};

use apollo::{lua_impl, lua_method};

use crate::api::carrier::{Carrier};
use crate::blake3::Hasher;
use crate::registry::{AnyRegistryBuilder, Registry, RegistryBuilder};
use crate::ty::{Prototype, Tag};

/// LuaRegistries contains all of the registry builders that this reload requires.
/// You can access a builder by the use of the `__index` meta-method which you can access like this.
/// ```lua
/// reload.registry["prototype-name"] -> LuaRegistryBuilder
/// ```
pub struct LuaRegistryBuilder {
	builders: HashMap<String, LuaRegistryDataBuilder>,
	hasher: Hasher,
}

impl LuaRegistryBuilder {
	pub fn new() -> LuaRegistryBuilder {
		LuaRegistryBuilder {
			builders: Default::default(),
			hasher: Default::default(),
		}
	}

	pub fn start_prototype<P: Prototype>(&mut self) {
		let name = P::lua_registry_name().to_string();
		let builder = LuaRegistryDataBuilder(Box::new(RegistryBuilder::<P>::new()));
		self.builders.insert(name, builder);
	}

	pub fn end_prototype<P: Prototype>(&mut self, carrier: Carrier) {
		carrier.data.write().registries.insert::<Registry<P>>(
			*self
				.builders
				.get(P::lua_registry_name())
				.expect("Prototypes builder missing, registration missing.")
				.0
				.finish(&mut self.hasher)
				.downcast::<Registry<P>>()
				.expect("wrong output type"),
		);
	}

	pub fn finish(self, carrier: Carrier) {
		carrier.data.write().hash = self.hasher.finalize();
	}
}

#[lua_impl]
impl LuaRegistryBuilder {
	#[lua_method]
	pub fn __index(&mut self, key: String) -> LuaResult<&mut LuaRegistryDataBuilder> {
		Ok(self
			.builders
			.get_mut(&key)
			.unwrap_or_else(|| panic!("Could not find registry named {key}")))
	}
}

/// A RegistryBuilder allows you to insert and extend prototypes in rustaria.
pub struct LuaRegistryDataBuilder(Box<dyn AnyRegistryBuilder>);

#[lua_impl]
impl LuaRegistryDataBuilder {
	#[lua_method]
	pub fn insert(&mut self, lua: &Lua, values: Table) -> LuaResult<()> {
		for res in values.pairs() {
			let (tag, table): (Tag, Table) = res?;
			self.0.register(lua, tag, table)?;
		}

		Ok(())
	}
}
