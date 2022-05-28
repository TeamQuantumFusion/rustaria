use std::fmt::Debug;
use item::{Item, ItemPrototype};

use rsa_core::api::carrier::Carrier;
use rsa_core::lua::{LuaUserData, LuaUserDataMethods};
use rsa_core::ty::{Prototype, Tag};

pub mod stack;
pub mod storage;
pub mod item;
mod testing;

pub struct ItemSystem {
	carrier: Carrier,
}

impl ItemSystem {
	pub fn get(&self, tag: &Tag) -> Option<Item> {
		self.carrier.get::<ItemPrototype>().create_from_tag(tag)
	}
}

impl LuaUserData for ItemSystem {
	fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("get", |_, system, tag| {
			Ok(system.get(&tag))
		});
	}
}

#[cfg(test)]
mod tests {
	use rsa_core::api::Api;
	use rsa_core::plugin::archive::{Archive, TestAsset};
	use rsa_core::plugin::Plugin;
	use rsa_core::reload;
	use rsa_core::error::Result;
	use rsa_core::lua::glue::{Glue};
	use rsa_core::lua::{LuaResult};
	use rsa_core::lua::error::LuaError;
	use crate::ItemSystem;
	use apollo::*;
	use crate::item::ItemPrototype;

	pub struct Game {
		item: ItemSystem
	}

	#[lua_impl]
	impl Game {
		#[lua_field]
		pub fn get_item(&mut self) -> LuaResult<&mut ItemSystem> {
			Ok(&mut self.item)
		}
	}

	#[test]
	fn it_works() -> Result<()> {
		rsa_core::initialize().unwrap();
		let mut api = Api::new_test();
		let carrier = api.get_carrier();

		let mut game = Game {
			item: ItemSystem {
				carrier
			}
		};


		api.load_test_plugins(vec![Plugin::new_test(
			"test",
			Archive::new_test(vec![TestAsset::lua(
				"entry", r#"
-- register
item:register {
	["stick"] = {
		max_stack = 10
	}
}

-- hook
hook:subscribe("rustaria:test", function(game)
	info("getting stick")
	local system = game.item;

	system:thing(69)

	local stick = system:get("stick")

	stick:thing(69)

	if stick then
		info "creating stack"
		local one_stick = stick:to_stack()
		local two_sticks = stick:to_stack(2)
		info(one_stick)
		info(tostring(two_sticks))
	end
end)"#,
			)]),
			&api,
		)]);
		reload!((ItemPrototype) => api);

		Glue::scope(&mut game, |glue| {
			api.invoke_hook("rustaria:test", || {
				glue
			}).lua_err().unwrap();
		});


		Ok(())
	}
}
