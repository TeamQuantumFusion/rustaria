use std::future::Future;
use std::marker::PhantomData;

use crate::{FromLuaMulti, Lua, LuaWeak, MetaMethod, ToLuaMulti, UserData, UserDataMethods};
use crate::types::MaybeSend;

pub struct GlueUserDataMethods<'a, V: UserData, I: UserDataMethods<LuaWeak<V>>> {
	pub(crate) methods: &'a mut I,
	pub(crate) data: PhantomData<V>,
}

impl<'a, V: UserData, I: UserDataMethods<LuaWeak<V>>> UserDataMethods<V>
	for GlueUserDataMethods<'a, V, I>
{
	fn add_method<S, A, R, M>(&mut self, name: &S, method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + Fn(&Lua, &V, A) -> eyre::Result<R>,
	{
		self.methods
			.add_method(name, move |lua, data, args| {
				method(lua, data.get("this")?, args)
			})
	}

	fn add_method_mut<S, A, R, M>(&mut self, name: &S, mut method: M)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> eyre::Result<R>,
	{
		self.methods
			.add_method_mut(name, move |lua, data, args| {
				method(lua, data.get_mut("this")?, args)
			})
	}

	#[cfg(feature = "async")]
	#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
	fn add_async_method<S, A, R, M, MR>(&mut self, name: &S, method: M)
		where
			T: Clone,
			S: AsRef<[u8]> + ?Sized,
			A: FromLuaMulti,
			R: ToLuaMulti,
			M: 'static + MaybeSend + Fn(&Lua, T, A) -> MR,
			MR: Future<Output = eyre::Result<R>> {
		todo!()
	}

	fn add_function<S, A, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + Fn(&Lua, A) -> eyre::Result<R>,
	{
		self.methods.add_function(name, function)
	}

	fn add_function_mut<S, A, R, F>(&mut self, name: &S, function: F)
	where
		S: AsRef<[u8]> + ?Sized,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + FnMut(&Lua, A) -> eyre::Result<R>,
	{
		self.methods.add_function_mut(name, function)
	}

	#[cfg(feature = "async")]
	#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
	fn add_async_function<S, A, R, F, FR>(&mut self, name: &S, function: F)
		where
			S: AsRef<[u8]> + ?Sized,
			A: FromLuaMulti,
			R: ToLuaMulti,
			F: 'static + MaybeSend + Fn(&Lua, A) -> FR,
			FR: Future<Output = eyre::Result<R>>  {
		todo!()
	}

	fn add_meta_method<S, A, R, M>(&mut self, meta: S, method: M)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + Fn(&Lua, &V, A) -> eyre::Result<R>,
	{
		self.methods
			.add_meta_method(meta, move |lua, data, args| {
				method(lua, data.get("this")?, args)
			})
	}

	fn add_meta_method_mut<S, A, R, M>(&mut self, meta: S, mut method: M)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		M: 'static + Send + FnMut(&Lua, &mut V, A) -> eyre::Result<R>,
	{
		self.methods
			.add_meta_method_mut(meta, move |lua, data, args| {
				method(lua, data.get_mut("this")?, args)
			})
	}

	fn add_meta_function<S, A, R, F>(&mut self, meta: S, function: F)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + Fn(&Lua, A) -> eyre::Result<R>,
	{
		self.methods.add_meta_function(meta, function);
	}

	fn add_meta_function_mut<S, A, R, F>(&mut self, meta: S, function: F)
	where
		S: Into<MetaMethod>,
		A: FromLuaMulti,
		R: ToLuaMulti,
		F: 'static + Send + FnMut(&Lua, A) -> eyre::Result<R>,
	{
		self.methods.add_meta_function_mut(meta, function);
	}
}
