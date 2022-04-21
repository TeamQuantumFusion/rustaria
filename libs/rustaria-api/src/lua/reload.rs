use crate::{info, Prototype, Registry, RegistryBuilder, Tag};
use eyre::eyre;
use mlua::{LuaSerdeExt, UserData, UserDataMethods, Value};
use rustaria_util::blake3::Hasher;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use eyre::Result;

#[derive(Clone, Default)]
pub struct RegistryBuilderLua<P: Prototype>(Arc<Mutex<RegistryBuilder<P>>>);

impl<P: Prototype> RegistryBuilderLua<P> {
	pub fn new() -> RegistryBuilderLua<P> {
		RegistryBuilderLua(Arc::new(Mutex::new(RegistryBuilder::new())))
	}

	pub fn collect(self, hasher: &mut Hasher) -> Result<Registry<P>> {
		self.0.lock().unwrap().finish(hasher)
	}
}

impl<P: Prototype> UserData for RegistryBuilderLua<P> {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("register", |lua, builder, values: HashMap<Tag, Value>| {
			for (tag, prototype) in values {
				info!("{tag:?}");

				builder
					.0
					.lock()
					.unwrap()
					.register(tag, lua.unpack(prototype)?);
			}

			Ok(())
		})
	}
}
