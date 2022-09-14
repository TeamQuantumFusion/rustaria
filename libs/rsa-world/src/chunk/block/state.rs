use apollo::FromLua;
use rsa_core::err::{ext::AuditExt, Result};
use rsa_registry::{Id, Identifier, IdValue, Prototype, Registry, RegistryBuilder, RegPointer};
use crate::chunk::block::ty::BlockType;

#[derive(Debug, Clone)]
pub struct BlockStates {
	pub states: Registry<BlockStateType>,
	pub default_state: Id<BlockStateType>,
}

impl BlockStates {
	pub fn get_block_state(
		&self,
		state: Option<RegPointer<BlockStateType>>,
	) -> Result<Id<BlockStateType>> {
		match state {
			None => Ok(self.default_state),
			Some(ptr) => self.states.get_id(ptr),
		}
	}
}

#[derive(FromLua, Debug)]
pub struct BlockStatesPrototype {
	pub states: RegistryBuilder<BlockStatePrototype>,
	pub default_state: Identifier,
}

impl BlockStatesPrototype {
	pub fn bake(self) -> Result<BlockStates> {
		let registry = self.states.build()?.map(|_, _, prototype| {
			Ok(BlockStateType {
				name: prototype.name,
			})
		})?;

		Ok(BlockStates {
			default_state: registry
				.lookup()
				.get_id(&self.default_state)
				.wrap_err_with(|| {
					format!("Default state {} does not exist", &self.default_state)
				})?,
			states: registry,
		})
	}
}

#[derive(Debug, Clone)]
pub struct BlockStateType {
	pub name: String,
}

impl IdValue for BlockStateType {
	type Idx = u16;
}

#[derive(FromLua, Debug, Clone)]
pub struct BlockStatePrototype {
	pub name: String,
}

impl Prototype for BlockStatePrototype {
	type Output = BlockStateType;

	fn get_name() -> &'static str {
		"block_state"
	}
}