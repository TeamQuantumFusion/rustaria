use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mlua::{UserData, UserDataMethods, Value};

use rustaria_util::blake3::Hasher;
use rustaria_util::error::Result;
use rustaria_util::logging::debug;

use crate::{Prototype, Registry, RegistryBuilder, Tag};

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
				debug!(target: "reload@rustaria.api", "Registered {tag:?}");

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
