use std::{collections::HashMap, time::Instant};

use anyways::{ext::AuditExt, Result};
use apollo::{macros::*, Lua, UserDataCell, Value};

use crate::api::{
	luna::lib::registry_builder::RegistryBuilder, prototype::Prototype, registry::Registry,
};

pub struct Stargate {
	pub start: Instant,
	pub builders: HashMap<String, Value>,
}

impl Stargate {
	pub fn new() -> Stargate {
		Stargate {
			start: Instant::now(),
			builders: Default::default(),
		}
	}

	pub fn register_builder<P: Prototype>(&mut self, lua: &Lua) -> Result<()> {
		self.builders.insert(
			P::get_name().to_string(),
			lua.pack(UserDataCell::new(RegistryBuilder::<P>::new()))
				.wrap_err_with(|| format!("Failed to convert {} Builder to Lua", P::get_name()))?,
		);

		Ok(())
	}

	pub fn build_registry<P: Prototype>(&mut self, _lua: &Lua) -> Result<Registry<P>> {
		let value = self
			.builders
			.remove(P::get_name())
			.expect("Registry unregistered");

		match value {
			Value::UserData(userdata) => {
				let builder: RegistryBuilder<P> = userdata.take().wrap_err("Wrong userdata")?;
				builder.build()
			}
			_ => panic!("not userdata"),
		}
	}
}

#[lua_impl]
/// The stargates job is to create a Carrier which holds all of the registries in rustaria.
/// # These are available on both client and server
/// @field block_layer RegistryBuilder<BlockLayerPrototype>
/// @field entity RegistryBuilder<EntityPrototype>
/// # These are only available on client
/// @field block_layer_renderer RegistryBuilder<BlockLayerRendererPrototype>
/// @field entity_renderer RegistryBuilder<EntityRendererPrototype>
impl Stargate {
	#[lua_method]
	pub fn __index(&mut self, name: String) -> Result<Value> {
		self.builders
			.get_mut(&name)
			.wrap_err_with(|| format!("Registry {} does not exist in this context.", name))
			.cloned()
	}
}