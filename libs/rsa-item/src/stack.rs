use crate::Item;

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