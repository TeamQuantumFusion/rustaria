use mlua::{UserDataFields, UserDataMethods, Value};
use type_map::TypeMap;
use eyre::{ContextCompat, WrapErr};
use log::{debug, trace};
use mlua::prelude::LuaUserData;
use crate::api::Api;
use crate::api::carrier::CarrierData;
use crate::blake3::Hasher;
use crate::lua::def::hook::HookInstanceBuilder;
use crate::lua::def::reload::RegistryBuilderLua;
use crate::registry::Registry;
use crate::ty::Prototype;

#[macro_export]
macro_rules! reload {
    (($($PROTOTYPE:ty),*) => $API:expr) => {
		let mut reload = $API.reload();
		$(reload.register::<$PROTOTYPE>()?;)*
		reload.reload()?;
		$(reload.collect::<$PROTOTYPE>()?;)*
		reload.apply();
    };
}

pub struct ApiReload<'a> {
	pub(crate) api: &'a mut Api,
	pub(crate) carrier: &'a mut CarrierData,
	pub(crate) registry_builders: TypeMap,
	pub(crate) hook_builder: HookInstanceBuilder,
	pub(crate) hasher: Hasher,
}

impl<'a> ApiReload<'a> {
	pub fn register<P: Prototype>(&mut self) -> eyre::Result<()> {
		debug!(target: "reload@rustaria.api",
			"Registered \"{}\" registry.",
			P::lua_registry_name()
		);

		let builder = RegistryBuilderLua::new();
		for (id, plugin) in &self.api.internals.read().unwrap().plugins {
			trace!(target: "reload@rustaria.api",
				"Registered \"{}\" registry to {id}.",
				P::lua_registry_name()
			);
			plugin
				.lua_state
				.globals()
				.set(P::lua_registry_name(), builder.clone())?;
		}

		self.registry_builders
			.insert::<RegistryBuilderLua<P>>(builder);
		Ok(())
	}

	pub fn reload(&mut self) -> eyre::Result<()> {
		for (id, plugin) in &self.api.internals.read().unwrap().plugins {
			plugin
				.lua_state
				.globals()
				.set("hook", self.hook_builder.lua())?;

			trace!(target: "reload@rustaria.api", "Reloading {id}");
			plugin
				.reload()
				.wrap_err(format!("Error while reloading plugin {id}"))?;

			plugin.lua_state.globals().set("hook", Value::Nil)?;
		}

		self.carrier.hash = self.hasher.finalize();

		Ok(())
	}

	pub fn collect<P: Prototype>(&mut self) -> eyre::Result<()> {
		debug!(target: "reload@rustaria.api",
			"Collecting \"{}\" registry.",
			P::lua_registry_name()
		);

		let builder = self
			.registry_builders
			.remove::<RegistryBuilderLua<P>>()
			.wrap_err(format!(
				"Could not find registry {}",
				P::lua_registry_name()
			))?;

		let registry = builder.collect(&mut self.hasher)?;

		self.carrier.registries.insert::<Registry<P>>(registry);
		Ok(())
	}

	pub fn apply(mut self) {
		self.api.write().hook_instance = self.hook_builder.export();
		self.carrier.hash = self.hasher.finalize();
	}
}

