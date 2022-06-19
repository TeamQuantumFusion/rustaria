mod fields;
mod methods;

use std::{
	error::Error,
	fmt::{Display, Formatter, Write},
	marker::PhantomData,
	ops::{Deref, DerefMut},
	sync::{Arc, Weak},
};
use std::sync::atomic::{AtomicBool, Ordering};

use eyre::ContextCompat;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
	scope::{fields::GlueUserDataFields, methods::GlueUserDataMethods},
	types::MaybeSend,
	FromLua, ToLua, UserData, UserDataFields, UserDataFromLua, UserDataMethods, UserDataToLua
};

pub struct LuaScope<'a, V> {
	alive: Arc<()>,
	value: *const V,
	mutable: bool,
	_lock: PhantomData<&'a V>,
}

impl<'a, V> LuaScope<'a, V> {
	unsafe fn new(value: *const V, mutable: bool) -> LuaScope<'a, V> {
		LuaScope {
			alive: Arc::new(()),
			value,
			mutable,
			_lock: Default::default(),
		}
	}

	pub fn lua(&self) -> LuaWeak<V> {
		LuaWeak {
			lock: Arc::downgrade(&self.alive),
			value: self.value,
			mutable: self.mutable,
		}
	}
}

impl<'a, V> From<&'a mut V> for LuaScope<'a, V> {
	fn from(value: &'a mut V) -> Self {
		unsafe {
			LuaScope::new(value, true)
		}
	}
}

impl<'a, V> From<&'a V> for LuaScope<'a, V> {
	fn from(value: &'a V) -> Self {
		unsafe {
			LuaScope::new(value, false)
		}
	}
}

impl<V> Drop for LuaScope<'_, V> {
	fn drop(&mut self) {
	}
}

pub struct LuaWeak<V> {
	lock: Weak<()>,
	value: *const V,
	mutable: bool,
}

impl<V> LuaWeak<V> {
	pub fn get(&self, local: &'static str) -> Result<&V, GlueError> {
		let _ = self.lock.upgrade().ok_or(GlueError::Dropped(local))?;
		unsafe { Ok(&*self.value) }
	}

	pub fn get_mut(&self, local: &'static str) -> Result<&mut V, GlueError> {
		if !self.mutable {
			return Err(GlueError::Immutable(local));
		}

		let _ = self.lock.upgrade().ok_or(GlueError::Dropped(local))?;
		unsafe { Ok(&mut *(self.value as *mut V)) }
	}

	pub unsafe fn extend<O>(&self, value: *const O, mutable: bool) -> Result<LuaWeak<O>, GlueError> {
		if mutable && !self.mutable {
			return Err(GlueError::ReturnMutable);
		}
		Ok(LuaWeak {
			lock: self.lock.clone(),
			value,
			mutable
		})
	}
}

unsafe impl<V: Send> Send for LuaWeak<V> {}
unsafe impl<V: Sync> Sync for LuaWeak<V> {}

impl<V> Clone for LuaWeak<V> {
	fn clone(&self) -> Self {
		LuaWeak {
			lock: self.lock.clone(),
			value: self.value,
			mutable: self.mutable,
		}
	}
}

impl<V: UserData> UserDataFromLua for LuaWeak<V> {}
impl<V: UserData> UserDataToLua for LuaWeak<V> {}
impl<V: UserData> UserData for LuaWeak<V> {
	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		V::add_fields(&mut GlueUserDataFields {
			fields,
			data: Default::default(),
		})
	}

	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		V::add_methods(&mut GlueUserDataMethods {
			methods,
			data: Default::default(),
		})
	}
}

#[derive(Debug)]
pub enum GlueError {
	Dropped(&'static str),
	Immutable(&'static str),
	ReturnMutable
}

impl Display for GlueError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			GlueError::Dropped(local) => {
				f.write_char('\"')?;
				f.write_str(local)?;
				f.write_char('\"')?;
				f.write_str(" is dropped.")?;
			}
			GlueError::Immutable(local) => {
				f.write_str("Tried to mutate \"")?;
				f.write_str(local)?;
				f.write_str("\" which is an immutable binding.")?;
			}
			GlueError::ReturnMutable => {
				f.write_str("Cannot return mutable when self is immutable. (we currently assume self is the only one you return a reference from)")?;
			}
		}
		Ok(())
	}
}

impl Error for GlueError {}
