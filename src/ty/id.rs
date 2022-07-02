use std::{
	hash::{Hash, Hasher},
	marker::PhantomData,
};

use crate::api::prototype::Prototype;
use apollo::macros::*;

/// The internal id is a instance bound identifier to the registry,
/// absolutely not forward/backwards compatible across versions or even game instances.
#[derive(Ord, PartialOrd, Debug, serde::Serialize, serde::Deserialize)]
pub struct Id<P> {
	id: u32,
	prototype: PhantomData<P>,
}

#[lua_impl]
impl<P> Id<P> {
	// DO NOT EVEN THINK ABOUT MAKING THIS. THIS SHOULD ONLY BE MADE FROM THE REGISTRY ITS FROM
	pub unsafe fn new(id: usize) -> Id<P> {
		Id {
			id: id as u32,
			prototype: Default::default(),
		}
	}

	pub fn index(&self) -> usize { self.id as usize }

	pub fn id(&self) -> u32 { self.id }
}

impl<P: Prototype> Id<P> {
	pub fn build(self) -> Id<P::Output> {
		Id {
			id: self.id,
			prototype: Default::default(),
		}
	}
}

unsafe impl<P> Send for Id<P> {

}
impl<P> Hash for Id<P> {
	fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) }
}
impl<P> PartialEq<Self> for Id<P> {
	fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<P> Eq for Id<P> {}

// This is needed as rustc cringes on the phantomdata
impl<P> Clone for Id<P> {
	fn clone(&self) -> Id<P> {
		Id {
			id: self.id,
			prototype: Default::default(),
		}
	}
}

impl<P> Copy for Id<P> {}
