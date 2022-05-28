use mlua::{AnyUserData, FromLua, Lua, MetaMethod, ToLua, UserData, UserDataFields};
use std::marker::PhantomData;
use crate::api::lua::glue::LuaGlue;

pub struct GlueUserDataFields<'a, V: UserData, F: UserDataFields<LuaGlue<V>>> {
	pub(crate) fields: &'a mut F,
	pub(crate) data: PhantomData<V>,
}

impl<'a, V: UserData, I: UserDataFields<LuaGlue<V>>> UserDataFields<V> for GlueUserDataFields<'a, V, I>
{
	fn add_field_method_get<S, R, M>(&mut self, name: &S, method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		R: ToLua,
		M: Send + 'static + Fn(&Lua, &V) -> mlua::Result<R>,
	{
		self.fields
			.add_field_method_get(name, move |lua, wrapper| unsafe {
				method(lua, &*wrapper.value)
			})
	}

	fn add_field_method_set<S, A, M>(&mut self, name: &S, mut method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLua,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> mlua::Result<()>,
	{
		self.fields
			.add_field_method_set(name, move |lua, wrapper, value| unsafe {
				method(lua, &mut *wrapper.value, value)
			})
	}

	fn add_field_function_get<S, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		R: ToLua,
		F: 'static + Send + Fn(&Lua, AnyUserData) -> mlua::Result<R>,
	{
		self.fields.add_field_function_get(name, function)
	}

	fn add_field_function_set<S, A, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLua,
		F: 'static + Send + FnMut(&Lua, AnyUserData, A) -> mlua::Result<()>,
	{
		self.fields.add_field_function_set(name, function)
	}

	fn add_meta_field_with<S, R, F>(&mut self, meta: S, f: F)
	where
		S: Into<MetaMethod>,
		F: 'static + Send + Fn(&Lua) -> mlua::Result<R>,
		R: ToLua,
	{
		self.fields.add_meta_field_with(meta, f)
	}
}
