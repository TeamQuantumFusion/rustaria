use std::fmt::Debug;
use std::marker::PhantomData;
use ahash::AHashMap;
use apollo::{FromLua, Lua, Value, macros::*};
use crate::{Identifier, Registry, RegistryKey};

pub const DEFAULT_PRIORITY: f32 = 1000.0;

#[derive(Debug)]
pub struct RegistryBuilder<P: FromLua + 'static> {
	pub values: AHashMap<Identifier, (f32, P)>,
	_p: PhantomData<P>,
}

#[lua_impl]
impl<P: FromLua + 'static + Debug> RegistryBuilder<P> {
	pub fn new() -> RegistryBuilder<P> {
		RegistryBuilder {
			values: AHashMap::default(),
			_p: Default::default(),
		}
	}

	#[lua_method]
	pub fn register(&mut self, values: Vec<(RegistryKey, P)>) {
		for (key, value) in values {
			self.values.insert(
				key.identifier,
				(key.priority.unwrap_or(DEFAULT_PRIORITY), value),
			);
		}
	}

	pub fn build(self) -> anyways::Result<Registry<P>> { Ok(Registry::new(self.values)) }
}

impl<P: FromLua + 'static + Debug> FromLua for RegistryBuilder<P> {
	fn from_lua(value: Value, lua: &Lua) -> anyways::Result<RegistryBuilder<P>> {
		let values: Vec<(RegistryKey, P)> = lua.unpack(value)?;
		let mut builder = RegistryBuilder::new();
		builder.register(values);
		Ok(builder)
	}
}