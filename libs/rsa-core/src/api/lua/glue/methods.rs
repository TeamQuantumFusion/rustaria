use crate::api::lua::glue::{LuaGlue};
use mlua::{FromLuaMulti, Lua, MetaMethod, ToLuaMulti, UserData, UserDataMethods};
use std::marker::PhantomData;

pub struct GlueUserDataMethods<'a, V: UserData, F: UserDataMethods<LuaGlue< V>>> {
	pub(crate) methods: &'a mut F,
	pub(crate) data: PhantomData<V>,
}

impl<'a, V: UserData, I: UserDataMethods<LuaGlue<V>>> UserDataMethods<V>
	for GlueUserDataMethods<'a, V, I>
{
	fn add_method<S, A, R, M>(&mut self, name: &S, method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + Fn(&Lua, &V, A) -> mlua::Result<R>,
	{
		self.methods.add_method(name, move |lua, data, args| unsafe  {
			method(lua, &*data.value, args)
		})
	}

	fn add_method_mut<S, A, R, M>(&mut self, name: &S, mut method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> mlua::Result<R>,
	{
		self.methods.add_method_mut(name, move |lua, data, args| unsafe  {
			method(lua, &mut *data.value, args)
		})
	}

	fn add_function<S, A, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + Fn(&Lua, A) -> mlua::Result<R>,
	{
		self.methods.add_function(name, function)
	}

	fn add_function_mut<S, A, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + FnMut(&Lua, A) -> mlua::Result<R>,
	{
		self.methods.add_function_mut(name, function)
	}

	fn add_meta_method<S, A, R, M>(&mut self, meta: S, method: M)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + Fn(&Lua, &V, A) -> mlua::Result<R>,
	{
		self.methods.add_meta_method(meta, move |lua, data, args| unsafe  {
			method(lua, &*data.value, args)
		})
	}

	fn add_meta_method_mut<S, A, R, M>(&mut self, meta: S, mut method: M)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> mlua::Result<R>,
	{
		self.methods.add_meta_method_mut(meta, move |lua, data, args| unsafe  {
			method(lua, &mut *data.value, args)
		})
	}

	fn add_meta_function<S, A, R, F>(&mut self, meta: S, function: F)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + Fn(&Lua, A) -> mlua::Result<R>,
	{
		self.methods.add_meta_function(meta, function);
	}

	fn add_meta_function_mut<S, A, R, F>(&mut self, meta: S, function: F)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + FnMut(&Lua, A) -> mlua::Result<R>,
	{
		self.methods.add_meta_function_mut(meta, function);
	}
}
