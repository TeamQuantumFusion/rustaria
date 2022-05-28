mod registry;
mod hook;

use eyre::Context;
use log::{info, trace};

use mlua::prelude::LuaResult;
use mlua::{Lua, Table, Value};
use parking_lot::RwLockWriteGuard;

use crate::api::carrier::{Carrier, CarrierData};
use crate::api::Api;
use apollo::*;
use registry::LuaRegistryBuilder;
use crate::api::reload::hook::LuaHookBuilder;

use crate::registry::{AnyRegistryBuilder, Registry, RegistryBuilder};
use crate::ty::{Prototype};
use crate::error::Result;
use crate::api::lua::glue::Glue;

#[macro_export]
macro_rules! reload {
    (($($PROTOTYPE:ty),*) => $API:expr) => {
		let mut reload = $API.reload();
		$(reload.start_prototype::<$PROTOTYPE>();)*
		reload.reload()?;
		$(reload.end_prototype::<$PROTOTYPE>();)*
		reload.finish();
    };
}

pub struct Reload<'a> {
	pub(crate) api: &'a mut Api,
	pub(crate) reload: LuaReload,
}

impl<'a> Reload<'a> {
	pub fn new(api: &'a mut Api) -> Reload<'a> {
		info!(target: "reload@rustaria.api", "Freezing carrier.");
		let carrier = api.get_carrier();
		let mut carrier = carrier.data.write();
		carrier.registries.clear();
		carrier.hash = [0; 32];
		Reload {
			api,
			reload: LuaReload::new()
		}
	}

	pub fn start_prototype<P: Prototype>(&mut self) {
		self.reload.registries.start_prototype::<P>();
	}

	pub fn reload(&mut self) -> Result<()> {
		for (id, plugin) in &self.api.internals.read().unwrap().plugins {
			let glue = Glue::new(&mut self.reload);
			plugin
				.lua_state
				.globals()
				.set("reload", glue.clone())?;

			trace!(target: "reload@rustaria.api", "Reloading {id}");
			plugin
				.reload()
				.wrap_err(format!("Error while reloading plugin {id}"))?;

			plugin.lua_state.globals().set("reload", Value::Nil)?;
		};

		Ok(())
	}

	pub fn end_prototype<P: Prototype>(&mut self) {
		let carrier = self.api.get_carrier();
		self.reload.registries.end_prototype::<P>(carrier);
	}

	pub fn finish(mut self) {
		let carrier = self.api.get_carrier();
		self.reload.registries.finish(carrier);
		self.reload.hooks.finish(&mut self.api.write().hook_instance);
	}
}

pub struct LuaReload {
	pub(crate) registries: LuaRegistryBuilder,
	pub(crate) hooks: LuaHookBuilder,
}

impl LuaReload {
	pub fn new() -> LuaReload {
		LuaReload {
			registries: LuaRegistryBuilder::new(),
			hooks: LuaHookBuilder::new()
		}
	}
}

#[lua_impl]
impl LuaReload {
	#[lua_field]
	pub fn get_registry(&mut self) -> LuaResult<&mut LuaRegistryBuilder> {
		Ok(&mut self.registries)
	}

	#[lua_field]
	pub fn get_hook(&mut self) -> LuaResult<&mut LuaHookBuilder> {
		Ok(&mut self.hooks)
	}
}
