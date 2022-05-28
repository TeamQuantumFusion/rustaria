use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::blake3::Hasher;
use crate::error::Result;
use crate::registry::{Registry, RegistryBuilder};
use crate::ty::{Prototype, Tag};
use mlua::{UserData, UserDataFields, UserDataMethods, Value};



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
