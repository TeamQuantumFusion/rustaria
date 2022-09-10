use std::mem::swap;

use crate::{stack::ItemStack, ItemAPI};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Storage {
	values: Vec<Option<ItemStack>>,
}

impl Storage {
	pub fn new(size: usize) -> Storage {
		let mut vec = Vec::with_capacity(size);
		for _ in 0..size {
			vec.push(None);
		}
		Storage { values: vec }
	}

	pub fn len(&self) -> usize { self.values.len() }

	pub fn get(&self, index: usize) -> Option<&ItemStack> { self.values.get(index)?.as_ref() }

	pub fn insert(&mut self, index: usize, item: ItemStack) -> Option<ItemStack> {
		self.values.get_mut(index).and_then(|old_value| {
			let mut out = Some(item);
			swap(old_value, &mut out);
			out
		})
	}

	pub fn add(&mut self, carrier: &ItemAPI, item: &mut ItemStack) {
		let desc = &carrier.item[item.item.id];

		for slot in &mut self.values {
			if let Some(slot) = slot {
				// check if the slot matches types
				if slot.item.id == item.item.id {
					let slots_remaining = desc.stack_size - slot.size;

					if slots_remaining >= item.size {
						// If the slots remaining is enough we set and return.
						slot.size += item.size;
						return;
					} else {
						// if items to insert is bigger than slots_remaining
						slot.size += slots_remaining;
						item.size -= slots_remaining;
					}
				}
			} else {
				// If the slot is empty just insert the item
				*slot = Some(ItemStack {
					item: item.item.clone(),
					size: item.size,
				});
				item.size = 0;
				return;
			}
		}
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut ItemStack> {
		self.values.get_mut(index)?.as_mut()
	}

	pub fn clear(&mut self) {
		for v in &mut self.values {
			*v = None;
		}
	}
}

#[cfg(all(test, feature = "testing"))]
mod tests {
	use rsa_core::{
		api::{reload::Reload, stargate::Stargate, Core},
		log::initialize,
		ty::Identifier,
	};

	use super::*;
	use crate::Item;

	fn init() -> (Core, ItemAPI) {
		initialize().unwrap();

		let mut rpc = ItemAPI::default();
		let mut core = Core::test_simple(
			r#"
		reload.stargate.item:register {
			["test:item-0"] = {
				name = "Testing stuff",
				stack_size = 100
			},
			["test:item-1"] = {
				name = "Testing stuff",
				stack_size = 100
			},
		}
		"#
			.to_string(),
		);

		let mut reload = Reload {
			stargate: Stargate::new(),
			client: false,
		};
		ItemAPI::register(&mut reload.stargate, &core.lua).unwrap();
		core.reload(&mut reload).unwrap();
		let rpc = ItemAPI::build(&mut reload.stargate).unwrap();
		(core, rpc)
	}

	#[test]
	fn test_insert() {
		let (api, carrier) = init();

		let item = carrier
			.item
			.get_id_from_identifier(&Identifier::test("item-0"))
			.unwrap();

		let mut storage = Storage::new(2);

		// Insert a single item
		let item = ItemStack::single(Item::new(item));
		storage.insert(0, item.clone());

		assert_eq!(
			storage,
			Storage {
				values: vec![Some(item.clone()), None,]
			}
		);
	}
	#[test]
	fn test_add() {
		let (api, carrier) = init();

		let item = Item::new(
			carrier
				.item
				.get_id_from_identifier(&Identifier::test("item-0"))
				.unwrap(),
		);

		let mut storage = Storage::new(2);

		storage.add(&carrier, &mut ItemStack::single(item.clone()));
		assert_eq!(
			storage,
			Storage {
				values: vec![Some(ItemStack::single(item.clone())), None,]
			}
		);

		storage.add(&carrier, &mut ItemStack::single(item.clone()));
		assert_eq!(
			storage,
			Storage {
				values: vec![Some(ItemStack::new(item, 2)), None,]
			}
		);
	}

	#[test]
	fn test_add_overflow() {
		let (api, carrier) = init();

		let item_0 = Item::new(
			carrier
				.item
				.get_id_from_identifier(&Identifier::test("item-0"))
				.unwrap(),
		);
		let item_1 = Item::new(
			carrier
				.item
				.get_id_from_identifier(&Identifier::test("item-1"))
				.unwrap(),
		);

		let mut storage = Storage::new(3);

		// set a different item in the middle
		storage.insert(1, ItemStack::single(item_1.clone()));

		// Fill nearly to max
		storage.add(&carrier, &mut ItemStack::new(item_0.clone(), 100));
		assert_eq!(
			storage,
			Storage {
				values: vec![
					Some(ItemStack::new(item_0.clone(), 100)),
					Some(ItemStack::single(item_1.clone())),
					None,
				]
			}
		);

		// Test overflow
		storage.add(&carrier, &mut ItemStack::new(item_0.clone(), 50));
		assert_eq!(
			storage,
			Storage {
				values: vec![
					Some(ItemStack::new(item_0.clone(), 100)),
					Some(ItemStack::single(item_1.clone())),
					Some(ItemStack::new(item_0.clone(), 50)),
				]
			}
		);

		// Test another kind of overflow
		storage.insert(0, ItemStack::new(item_0.clone(), 99));
		storage.add(&carrier, &mut ItemStack::new(item_0.clone(), 50));
		assert_eq!(
			storage,
			Storage {
				values: vec![
					Some(ItemStack::new(item_0.clone(), 100)),
					Some(ItemStack::single(item_1.clone())),
					Some(ItemStack::new(item_0.clone(), 99)),
				]
			}
		);
	}

	#[test]
	fn test_fill() {
		let (api, carrier) = init();
		let item = Item::new(
			carrier
				.item
				.get_id_from_identifier(&Identifier::test("item-0"))
				.unwrap(),
		);

		let mut storage = Storage::new(1);
		storage.add(&carrier, &mut ItemStack::new(item.clone(), 50));
		let mut stack = ItemStack::new(item.clone(), 100);
		storage.add(&carrier, &mut stack);

		assert_eq!(
			storage,
			Storage {
				values: vec![Some(ItemStack::new(item.clone(), 100))]
			}
		);
		assert_eq!(stack, ItemStack::new(item.clone(), 50))
	}

	#[test]
	fn test_clear() {
		let (api, carrier) = init();
		let item = Item::new(
			carrier
				.item
				.get_id_from_identifier(&Identifier::test("item-0"))
				.unwrap(),
		);

		let mut storage = Storage::new(1);
		storage.add(&carrier, &mut ItemStack::new(item.clone(), 50));
		assert_eq!(
			storage,
			Storage {
				values: vec![Some(ItemStack::new(item.clone(), 50))]
			}
		);

		storage.clear();
		assert_eq!(storage, Storage { values: vec![None] });
	}
}
