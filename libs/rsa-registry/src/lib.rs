#![feature(type_alias_impl_trait)]
pub use crate::{
	id::{Id, IdValue},
	prototype::Prototype,
	identifier::Identifier,
	registry::{builder::RegistryBuilder, key::RegistryKey, lookup::RegistryLookup, Registry},
	storage::Storage,
};

mod id;
mod identifier;
mod registry;
mod storage;
mod prototype;

#[derive(Debug, Clone, Copy)]
pub enum RegPointer<'a, V: IdValue>  {
	Identifier(&'a Identifier),
	Raw(Id<V>)
}

impl<'a, V: IdValue> From<&'a Identifier> for RegPointer<'a, V> {
	fn from(ident: &'a Identifier) -> Self {
		RegPointer::Identifier(ident)
	}
}

impl<'a, V: IdValue> From<Id<V>> for RegPointer<'a, V> {
	fn from(id: Id<V>) -> Self {
		RegPointer::Raw(id)
	}
}

