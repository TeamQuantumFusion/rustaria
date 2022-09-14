use std::fmt::Debug;
use std::marker::PhantomData;
use ahash::AHashMap;
use anyways::ext::AuditExt;
use apollo::{FromLua, Lua, Value, macros::*};
use crate::{Identifier, IdValue, Prototype, Registry, RegistryKey};

pub const DEFAULT_PRIORITY: f32 = 1000.0;

#[derive(Debug)]
pub struct RegistryBuilder<P: FromLua + 'static + IdValue> {
	pub values: AHashMap<Identifier, (f32, P)>,
	_p: PhantomData<P>,
}

#[lua_impl]
impl<P: FromLua + 'static + Debug + IdValue> RegistryBuilder<P> {
	pub fn new() -> RegistryBuilder<P> {
		RegistryBuilder {
			values: AHashMap::default(),
			_p: Default::default(),
		}
	}


	#[lua_method]
	pub fn register(&mut self, key: RegistryKey, value: P) {
		self.values.insert(
			key.identifier,
			(key.priority.unwrap_or(DEFAULT_PRIORITY), value),
		);
	}

	pub fn build(self) -> anyways::Result<Registry<P>> { Ok(Registry::new(self.values)) }
}

impl<P: FromLua + 'static + Debug + IdValue> FromLua for RegistryBuilder<P> {
	/// table<RegistryKey, P>
	fn from_lua(value: Value, lua: &Lua) -> anyways::Result<RegistryBuilder<P>> {
		let values: Vec<(RegistryKey, Value)> = lua.unpack(value)?;
		let mut builder = RegistryBuilder::new();
		for (key, value) in values {
			let value: P = lua.unpack::<P>(value).wrap_err_with(|| format!("Failed to register {}", key.identifier))?;
			builder.register(key, value);
		}
		Ok(builder)
	}
}