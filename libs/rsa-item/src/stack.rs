use rsa_core::api::carrier::Carrier;
#[allow(unused_imports)]
use rsa_core::lua::{Lua, LuaMetaMethod, LuaResult, LuaUserData, LuaUserDataMethods};
use rsa_core::lua::get_api;
use crate::item::Item;
use apollo::{lua_method, lua_impl};
use crate::ItemPrototype;


/// An ItemStack is an item or more.
#[derive(Clone, PartialOrd, PartialEq)]
pub struct ItemStack {
	/// The item this is.
	item: Item,
	/// The amount of items that are in this "slot"
	amount: u32,
}

impl ItemStack {
	pub fn new(item: Item, amount: Option<u32>) -> ItemStack {
		ItemStack { item, amount: amount.unwrap_or(1) }
	}

	pub fn increase(&mut self, carrier: &Carrier, amount: u32) -> Option<u32> {
		let prototype = carrier.get::<ItemPrototype>().prototype_from_id(self.item.ty);
		if self.amount + amount <= prototype.max_stack {
			self.amount += amount;
			None
		} else {
			let leftover = (self.amount + amount) - prototype.max_stack;
			self.amount = prototype.max_stack;
			Some(leftover)
		}
	}
}

#[lua_impl]
impl ItemStack {
	#[lua_method(increase)]
	pub fn lua_increase(&mut self, lua: &Lua, amount: u32) -> LuaResult<Option<u32>> {
		Ok(self.increase(&get_api(lua).get_carrier(), amount))
	}

	#[lua_method]
	pub fn __tostring(&self, lua: &Lua) -> LuaResult<String> {
		Ok(format!("(item: {}, amount: {})", self.item.__tostring(lua)?, self.amount))
	}
}
