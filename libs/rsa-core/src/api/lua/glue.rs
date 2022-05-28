use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use mlua::{
	UserData, UserDataFields, UserDataMethods,
};
use fields::GlueUserDataFields;
use crate::api::lua::glue::methods::GlueUserDataMethods;

mod fields;
mod methods;

pub trait ToGlue where Self: UserData {
	fn glue(&mut self) -> Glue<Self>;
}

impl<V: UserData> ToGlue for V  {
	fn glue(&mut self) -> Glue<Self> {
		Glue::new(self)
	}
}

#[derive(Clone)]
pub struct Glue<'a, V: UserData> {
	raw: LuaGlue<V>,
	_value: PhantomData<&'a mut V>,
}

impl<'a, V: UserData> Glue<'a, V>  {
	pub fn new(data: &'a mut V) -> Glue<'a, V>  {
		Glue {
			raw: LuaGlue {
				value: data
			},
			_value: PhantomData::default()
		}
	}

	pub fn lua(self) -> LuaGlue<V> {
		self.raw
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
	pub fn new_raw(value: *mut V) -> LuaGlue<V> {
		LuaGlue {
			value
		}
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