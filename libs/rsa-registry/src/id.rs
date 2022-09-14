use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use num::cast::AsPrimitive;
use num::{FromPrimitive, PrimInt, ToPrimitive};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Id<V: IdValue> {
	pub(crate) key: V::Idx,
	pub(crate) _v: PhantomData<V>
}


impl<V: IdValue> Id<V> {
	pub(crate) fn new(key: V::Idx) -> Id<V> {
		Id {
			key,
			_v: Default::default()
		}
	}

	pub(crate) fn cast<O: IdValue>(self) -> Id<O> {
		Id {
			key: O::Idx::from_usize(self.key.to_usize().unwrap()).unwrap(),
			_v: Default::default()
		}
	}

	#[inline(always)]
	pub fn index(&self) -> usize {
		self.key.to_usize().unwrap()
	}
}

unsafe impl<V: IdValue> Send for Id<V> {}
unsafe impl<V: IdValue> Sync for Id<V> {}

impl<V: IdValue> Hash for Id<V> {
	fn hash<H: Hasher>(&self, state: &mut H) { self.key.hash(state) }
}

impl<V: IdValue> PartialEq<Self> for Id<V> {
	fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<V: IdValue> Eq for Id<V> {}

impl<V: IdValue> Debug for Id<V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "ID:{}", self.key) }
}

// This is needed as rustc cringes on the phantomdata
impl<V: IdValue> Clone for Id<V> {
	fn clone(&self) -> Id<V> {
		Id {
			key: self.key,
			_v: Default::default(),
		}
	}
}

impl<V: IdValue> Copy for Id<V> {

}

pub trait IdValue {
	type Idx: PrimInt + Display + AsPrimitive<usize> + FromPrimitive + Hash;
}