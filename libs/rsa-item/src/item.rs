use crate::stack::ItemStack;
use apollo::*;
use rsa_core::api::lua::FromLua;
use rsa_core::api::lua::{get_meta, Lua, LuaResult};
#[allow(unused_imports)]
use rsa_core::ty::{Prototype, RawId};

#[derive(Clone, Debug, serde::Deserialize, FromLua)]
pub struct ItemPrototype {
	pub max_stack: u32,
}

impl Prototype for ItemPrototype {
	type Item = Item;

	fn create(&self, id: RawId) -> Self::Item {
		Item { ty: id }
	}

	fn lua_registry_name() -> &'static str {
		"item"
	}
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Item {
	pub ty: RawId,
}

#[lua_impl]
impl Item {
	#[lua_method]
	pub fn to_stack(&self, amount: Option<u32>) -> LuaResult<ItemStack> {
		Ok(ItemStack::new(self.clone(), amount))
	}

	#[lua_method]
	pub fn thing(&self, number: u32) -> LuaResult<()> {
		println!("{}", number);
		Ok(())
	}

	#[lua_method]
	pub fn __tostring(&self, lua: &Lua) -> LuaResult<String> {
		Ok(get_meta(lua)
			.api
			.get_carrier()
			.get::<ItemPrototype>()
			.tag_from_id(self.ty)
			.to_string())
	}
}
