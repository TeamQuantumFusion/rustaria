use std::{collections::HashMap, time::Instant};

use apollo::impl_macro::*;
use eyre::{ContextCompat, WrapErr};
use apollo::{Lua};
use eyre::Result;

use crate::{
	api::{
		luna::lib::registry_builder::{DynRegistryBuilder, LuaRegistryBuilder, RegistryBuilder},
		prototype::Prototype,
		registry::Registry,
	},
};

/// Creates a carrier
pub struct Stargate {
	pub start: Instant,
	pub builders: HashMap<String, LuaRegistryBuilder>,
}

impl Stargate {
	pub fn new() -> Stargate {
		Stargate {
			start: Instant::now(),
			builders: Default::default(),
		}
	}

	pub fn register_builder<P: Prototype>(&mut self) {
		self.builders.insert(
			P::get_name().to_string(),
			LuaRegistryBuilder::new(RegistryBuilder::<P>::new()),
		);
	}

	pub fn build_registry<P: Prototype>(
		&mut self,
		lua: &Lua,
	) -> eyre::Result<Registry<P>> {
		self.builders
			.remove(P::get_name())
			.expect("Registry unregistered")
			.build(lua)
			.wrap_err_with(|| format!("Failed to create registry for {}s", P::get_name()))
	}
}

#[lua_impl]
impl Stargate {
	#[lua_method]
	pub fn __index(&mut self, name: String) -> Result<&mut LuaRegistryBuilder> {
		self.builders.get_mut(&name).wrap_err_with(|| format!("Registry {} does not exist in this context.", name))
	}
}