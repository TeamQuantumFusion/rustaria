use apollo::FromLua;
use crate::api::prototype::Prototype;
use crate::item::ItemDesc;

#[derive(FromLua, Debug)]
pub struct ItemPrototype {
	pub name: String,
	pub stack_size: u32
}

impl ItemPrototype {
	pub fn bake(self) -> ItemDesc {
		ItemDesc {
			name: self.name,
			stack_size: self.stack_size
		}
	}
}

impl Prototype for ItemPrototype {
	type Output = ItemDesc;

	fn get_name() -> &'static str {
		"item"
	}
}