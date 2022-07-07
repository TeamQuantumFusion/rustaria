use std::marker::PhantomData;

use crate::{AnyUserData, FromLua, Lua, LuaWeak, MetaMethod, ToLua, UserData, UserDataFields};

pub struct GlueUserDataFields<'a, V: UserData, F: UserDataFields<LuaWeak<V>>> {
	pub(crate) fields: &'a mut F,
	pub(crate) data: PhantomData<V>,
}

impl<'a, V: UserData, I: UserDataFields<LuaWeak<V>>> UserDataFields<V>
	for GlueUserDataFields<'a, V, I>
{
	fn add_field_method_get<S, R, M>(&mut self, name: &S, method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		R: ToLua,
		M: Send + 'static + Fn(&Lua, &V) -> eyre::Result<R>,
	{
		self.fields
			.add_field_method_get(name, move |lua, wrapper| unsafe {
				method(lua, wrapper.get("this")?)
			})
	}

	fn add_field_method_set<S, A, M>(&mut self, name: &S, mut method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLua,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> eyre::Result<()>,
	{
		self.fields
			.add_field_method_set(name, move |lua, wrapper, value| unsafe {
				method(lua, wrapper.get_mut("this")?, value)
			})
	}

	fn add_field_function_get<S, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		R: ToLua,
		F: 'static + Send + Fn(&Lua, AnyUserData) -> eyre::Result<R>,
	{
		self.fields.add_field_function_get(name, function)
	}

	fn add_field_function_set<S, A, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLua,
		F: 'static + Send + FnMut(&Lua, AnyUserData, A) -> eyre::Result<()>,
	{
		self.fields.add_field_function_set(name, function)
	}

	fn add_meta_field_with<S, R, F>(&mut self, meta: S, f: F)
	where
		S: Into<MetaMethod>,
		F: 'static + Send + Fn(&Lua) -> eyre::Result<R>,
		R: ToLua,
	{
		self.fields.add_meta_field_with(meta, f)
	}
}
