// mod fields;
// mod methods;

use std::{
	error::Error,
	fmt::{Display, Formatter, Write},
	marker::PhantomData,
	sync::{Arc},
};
use std::any::{Any};
use std::rc::Rc;

use serde::{Deserializer, Serializer};
use crate::UserData;

use crate::userdata::{UserDataCell};

pub struct LuaScope<'a, V: 'static + UserData> {
	// Box<dyn Any> is always ()
	alive: Rc<dyn Any>,
	value: *const V,
	mutable: bool,
	_lock: PhantomData<&'a V>,
}

impl<'a, V: 'static + UserData> LuaScope<'a, V> {
	unsafe fn new(value: *const V, mutable: bool) -> LuaScope<'a, V> {
		LuaScope {
			alive: Rc::new(Box::new(())),
			value,
			mutable,
			_lock: Default::default(),
		}
	}

	pub fn lua(&self) -> UserDataCell<V> {
		UserDataCell::Reference {
			reference: self.value,
			lock: Rc::downgrade(&self.alive),
			mutable: self.mutable,
		}
	}
}

impl<'a, V: 'static + UserData> From<&'a mut V> for LuaScope<'a, V> {
	fn from(value: &'a mut V) -> Self {
		unsafe {
			LuaScope::new(value, true)
		}
	}
}

impl<'a, V: 'static + UserData> From<&'a V> for LuaScope<'a, V> {
	fn from(value: &'a V) -> Self {
		unsafe {
			LuaScope::new(value, false)
		}
	}
}

impl<V: 'static + UserData> Drop for LuaScope<'_, V> {
	fn drop(&mut self) {
	}
}

// pub struct LuaWeak<V> {
// 	lock: Weak<()>,
// 	value: *const V,
// 	mutable: bool,
// }
//
// impl<V> LuaWeak<V> {
// 	pub fn get(&self, local: &'static str) -> Result<&V, RefError> {
// 		let _ = self.lock.upgrade().ok_or(RefError::Dropped(local))?;
// 		unsafe { Ok(&*self.value) }
// 	}
//
// 	pub fn get_mut(&self, local: &'static str) -> Result<&mut V, RefError> {
// 		if !self.mutable {
// 			return Err(RefError::Immutable(local));
// 		}
//
// 		let _ = self.lock.upgrade().ok_or(RefError::Dropped(local))?;
// 		unsafe { Ok(&mut *(self.value as *mut V)) }
// 	}
//
// 	pub unsafe fn extend<O>(&self, value: *const O, mutable: bool) -> Result<LuaWeak<O>, RefError> {
// 		if mutable && !self.mutable {
// 			return Err(RefError::ReturnMutable);
// 		}
// 		Ok(LuaWeak {
// 			lock: self.lock.clone(),
// 			value,
// 			mutable
// 		})
// 	}
// }
//
// unsafe impl<V: Send> Send for LuaWeak<V> {}
//
// impl<V> Clone for LuaWeak<V> {
// 	fn clone(&self) -> Self {
// 		LuaWeak {
// 			lock: self.lock.clone(),
// 			value: self.value,
// 			mutable: self.mutable,
// 		}
// 	}
// }
//
// impl<V: UserData> UserData for LuaWeak<V> {
// 	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
// 		V::add_fields(&mut GlueUserDataFields {
// 			fields,
// 			data: Default::default(),
// 		});
//
// 		fields.add_field_method_get("__mut", |lua, weak| {
// 			Ok(weak.mutable)
// 		});
//
// 		fields.add_field_method_get("__type", |lua, weak| {
// 			Ok(type_name::<V>())
// 		});
// 	}
//
// 	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
// 		V::add_methods(&mut GlueUserDataMethods {
// 			methods,
// 			data: Default::default(),
// 		})
// 	}
// }

#[derive(Clone, Debug)]
pub enum RefError {
	Dropped(&'static str),
	Immutable(&'static str),
	MutablyBorrowLocked(&'static str),
	NotOwned,
	Locked,
	ReturnMutable
}

impl Display for RefError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RefError::Dropped(local) => {
				f.write_char('\"')?;
				f.write_str(local)?;
				f.write_char('\"')?;
				f.write_str(" is dropped.")?;
			}
			RefError::Immutable(local) => {
				f.write_str("Tried to mutate \"")?;
				f.write_str(local)?;
				f.write_str("\" which is an immutable binding.")?;
			}
			RefError::MutablyBorrowLocked(local) => {
				write!(f, "Cannot borrow {local} because it is already mutably borrowed")?;
			}
			RefError::ReturnMutable => {
				f.write_str("Cannot return mutable when self is immutable.")?;
			}
			RefError::NotOwned => {
				write!(f, "The value is not a lua owned value.")?;
			}
			RefError::Locked => {
				write!(f, "The value is still locked.")?;

			}
		}
		Ok(())
	}
}

impl Error for RefError {}
