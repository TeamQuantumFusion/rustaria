pub mod state;
pub mod prototype;

use std::collections::HashMap;

use apollo::{FromLua, macros::*};
use rsa_core::{
	api::prototype::Prototype,
	err::Result,
};
use rsa_registry::Id;

use crate::{AuditExt, BlockLayer, spread::block::{BlockSpreader, BlockSpreaderPrototype}};
use crate::chunk::block::state::BlockStates;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Block {
	pub id: Id<BlockDesc>,
	pub layer: Id<BlockLayer>,
	pub collision: bool,
}

#[lua_impl]
impl Block {
	#[lua_method]
	pub fn get_collision(&self) -> bool { self.collision }
}

#[derive(Clone)]
pub struct BlockDesc {
	pub collision: bool,
	pub spread: Option<BlockSpreader>,
}

#[lua_impl]
impl BlockDesc {
	pub fn create(&self, id: Id<BlockDesc>, layer: Id<BlockLayer>) -> Block {
		Block {
			id,
			layer,
			collision: self.collision,
		}
	}
}
