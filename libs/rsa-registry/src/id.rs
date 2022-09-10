use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Id<V> {
	pub(crate) key: u32,
	pub(crate) _v: PhantomData<V>
}


impl<V> Id<V> {
	pub(crate) fn new(key: u32) -> Id<V> {
		Id {
			key,
			_v: Default::default()
		}
	}

	pub(crate) fn cast<O>(self) -> Id<O> {
		Id {
			key: self.key,
			_v: Default::default()
		}
	}

	#[inline(always)]
	pub fn index(&self) -> usize {
		self.key as usize
	}
}

unsafe impl<V> Send for Id<V> {}
unsafe impl<V> Sync for Id<V> {}

impl<V> Hash for Id<V> {
	fn hash<H: Hasher>(&self, state: &mut H) { self.key.hash(state) }
}

impl<V> PartialEq<Self> for Id<V> {
	fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<V> Eq for Id<V> {}

impl<V> Debug for Id<V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "ID:{}", self.key) }
}

// This is needed as rustc cringes on the phantomdata
impl<V> Clone for Id<V> {
	fn clone(&self) -> Id<V> {
		Id {
			key: self.key,
			_v: Default::default(),
		}
	}
}

impl<V> Copy for Id<V> {}