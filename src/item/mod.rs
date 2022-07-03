//! # Item API
use crate::ty::id::Id;
use apollo::{macros::lua_impl};

pub mod storage;
pub mod prototype;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
	pub id: Id<ItemDesc>
}

impl Item {
	pub fn new(id: Id<ItemDesc>) -> Item {
		Item {
			id
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemStack {
	pub item: Item,
	pub size: u32,
}

impl ItemStack {
	pub fn single(item: Item) -> ItemStack {
		ItemStack {
			item,
			size: 1
		}
	}

	pub fn new(item: Item, size: u32) -> ItemStack {
		ItemStack {
			item,
			size
		}
	}
}

pub struct ItemDesc {
	pub name: String,
	pub stack_size: u32
}

#[lua_impl]
impl ItemDesc {}

#[cfg(all(test, feature = "testing"))]
pub(crate) mod tests {

}