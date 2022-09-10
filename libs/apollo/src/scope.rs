// mod fields;
// mod methods;

use std::{
	any::Any,
	error::Error,
	fmt::{Display, Formatter, Write},
	marker::PhantomData,
	rc::Rc,
};
use std::any::type_name;
use log::trace;

use crate::{userdata::UserDataCell, UserData};

pub struct LuaScope<'a, V: 'static + UserData> {
	// Box<dyn Any> is always ()
	alive: Rc<dyn Any>,
	value: *const V,
	mutable: bool,
	_lock: PhantomData<&'a V>,
}

impl<'a, V: 'static + UserData> LuaScope<'a, V> {
	unsafe fn new(value: *const V, mutable: bool) -> LuaScope<'a, V> {
		trace!("Created scope for {}", type_name::<V>());

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
	fn from(value: &'a mut V) -> Self { unsafe { LuaScope::new(value, true) } }
}

impl<'a, V: 'static + UserData> From<&'a V> for LuaScope<'a, V> {
	fn from(value: &'a V) -> Self { unsafe { LuaScope::new(value, false) } }
}

impl<V: 'static + UserData> Drop for LuaScope<'_, V> {
	fn drop(&mut self) {
		trace!("Dropped scope for {}", type_name::<V>())
	}
}

#[derive(Clone, Debug)]
pub enum RefError {
	Dropped(&'static str),
	Immutable(&'static str),
	MutablyBorrowLocked(&'static str),
	NotOwned,
	Locked,
	ReturnMutable,
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
				write!(
					f,
					"Cannot borrow {local} because it is already mutably borrowed"
				)?;
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
