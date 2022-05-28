use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use mlua::{
	UserData, UserDataFields, UserDataMethods,
};
use fields::GlueUserDataFields;
use crate::lua::glue::methods::GlueUserDataMethods;

mod fields;
mod methods;

pub struct Glue<'a, V: UserData> {
	raw: LuaGlue<V>,
	_value: PhantomData<&'a mut V>,
}

impl<'a, V: UserData> Glue<'a, V>  {
	pub fn scope(data: &'a mut V, func: impl FnOnce(LuaGlue<V>)) {
		let glue = Glue {
			raw: LuaGlue {
				value: data
			},
			_value: PhantomData::default()
		};

		func((&*glue).clone())
	}

}

impl<'a, V: UserData> Deref for Glue<'a, V> {
	type Target = LuaGlue<V>;

	fn deref(&self) -> &Self::Target {
		&self.raw
	}
}

impl<'a, V: UserData> DerefMut for Glue<'a, V> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.raw
	}
}

pub struct LuaGlue<V: UserData> {
	value: *mut V,
}
impl<V: UserData> LuaGlue<V> {
	pub unsafe fn new(value: &V) -> LuaGlue<V> {
		LuaGlue {
			value: value as *const V as *mut V
		}
	}

	pub fn new_raw(value: *mut V) -> LuaGlue<V> {
		LuaGlue {
			value
		}
	}

	pub unsafe fn get_mut(&mut self) -> &mut V {
		&mut *self.value
	}
}

impl<V: UserData> Clone for LuaGlue<V> {
	fn clone(&self) -> Self {
		LuaGlue {
			value: self.value
		}
	}
}

unsafe impl<V: UserData> Send for LuaGlue<V> {}

impl<V: UserData> UserData for LuaGlue<V> {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		V::add_fields(&mut GlueUserDataFields { fields, data: Default::default(), })
	}

	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		V::add_methods(&mut GlueUserDataMethods { methods, data: Default::default(), })
	}
}