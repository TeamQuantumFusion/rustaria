//! # Item API

use apollo::{macros::*, Lua};
use rsa_core::{api::stargate::Stargate, err::Result};
use rsa_hash::Hasher;
use rsa_registry::{Id, IdValue, Registry};

use crate::prototype::ItemPrototype;

pub mod prototype;
pub mod stack;
pub mod storage;

#[derive(Default)]
pub struct ItemAPI {
	pub item: Registry<ItemType>,
}

#[lua_impl]
impl ItemAPI {
	#[lua_field(get item)]
	pub fn item(&self) -> &Registry<ItemType> {
		&self.item
	}

	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		stargate.register_builder::<ItemPrototype>(lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<ItemAPI> {
		Ok(ItemAPI {
			item: stargate
				.build_registry::<ItemPrototype>()?
				.map(|_, _, prototype| Ok(prototype.bake()))?,
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) { self.item.lookup().append_hasher(hasher); }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
	pub id: Id<ItemType>,
}

impl Item {
	pub fn new(id: Id<ItemType>) -> Item { Item { id } }
}

pub struct ItemType {
	pub name: String,
	pub stack_size: u32,
}

impl IdValue for ItemType {
	type Idx = u32;
}

#[lua_impl]
impl ItemType {}

#[cfg(all(test, feature = "testing"))]
pub(crate) mod tests {}
