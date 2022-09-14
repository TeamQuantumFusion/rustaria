use apollo::FromLua;
use rsa_registry::{IdValue, Prototype};

use crate::ItemType;

#[derive(FromLua, Debug)]
pub struct ItemPrototype {
	pub name: String,
	pub stack_size: u32,
}

impl ItemPrototype {
	pub fn bake(self) -> ItemType {
		ItemType {
			name: self.name,
			stack_size: self.stack_size,
		}
	}
}

impl Prototype for ItemPrototype {
	type Output = ItemType;

	fn get_name() -> &'static str { "item" }
}
