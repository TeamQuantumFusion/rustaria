use rsa_core::error::{bail, Result};
use crate::stack::ItemStack;

/// An ItemStorage is a fixed size storage for items. It has ´len()´ amount of slots that is either `None` or `Some(ItemStack)`
pub struct ItemStorage {
	items: Vec<Option<ItemStack>>,
}

impl ItemStorage {
	/// Create a new ItemStorage with `size` amount of slots
	pub fn new(size: u32) -> ItemStorage {
		ItemStorage {
			items: vec![None; size as usize],
		}
	}

	/// Get the length of the ItemStorage
	pub fn len(&self) -> u32 {
		self.items.len() as u32
	}

	/// Checks if the Storage is empty
	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	/// Get a slot on `pos`
	pub fn get(&self, pos: u32) -> Option<&ItemStack> {
		self.items.get(pos as usize)?.as_ref()
	}

	/// Set a slot on `pos`
	pub fn set(&mut self, pos: u32, stack: Option<ItemStack>) -> Result<()> {
		let index = pos as usize;
		let len = self.items.len();
		if index > len {
			bail!("Out of bounds for storage size {}", len)
		}
		self.items.insert(index, stack);
		Ok(())
	}
}
