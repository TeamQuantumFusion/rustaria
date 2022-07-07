//! # Item API

use apollo::Lua;
use apollo::macros::*;
use rsa_core::{
	api::{
		reload::{Reload, RustariaPrototypeCarrier},
		Core,
	},
	blake3::Hasher,
	err::Result,
	ty::{Id, Registry},
};
use rsa_core::api::stargate::Stargate;

use crate::prototype::ItemPrototype;

pub mod prototype;
pub mod stack;
pub mod storage;

#[derive(Default)]
pub struct ItemRPC {
	pub item: Registry<ItemDesc>,
}

impl ItemRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<ItemPrototype>(lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<ItemRPC> {
		Ok(ItemRPC {
			item: stargate.build_registry::<ItemPrototype>()?
				.into_entries()
				.map(|(id, ident, prototype)| (id.build(), ident, prototype.bake()))
				.collect()
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.item.append_hasher(hasher);
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
	pub id: Id<ItemDesc>,
}

impl Item {
	pub fn new(id: Id<ItemDesc>) -> Item { Item { id } }
}

pub struct ItemDesc {
	pub name: String,
	pub stack_size: u32,
}

#[lua_impl]
impl ItemDesc {}

#[cfg(all(test, feature = "testing"))]
pub(crate) mod tests {}
