use std::collections::HashMap;

use apollo::{FromLua, macros::*};
use rand::Rng;
use rand_xoshiro::Xoroshiro64Star;
use rsa_core::{
	err::{ext::AuditExt, Result},
	TPS,
	ty::{Direction, Offset},
};
use rsa_registry::{Id, Identifier, RegistryLookup};

use crate::{BlockPos, ChunkLayerType, ChunkStorage};
use crate::chunk::block::ty::BlockType;

#[derive(Clone)]
pub struct BlockSpreader {
	pub chance: f32,
	pub convert_table: HashMap<Id<BlockType>, Id<BlockType>>,
}

impl BlockSpreader {
	pub fn tick_spread(
		&self,
		pos: BlockPos,
		layer_id: Id<ChunkLayerType>,
		chunks: &mut ChunkStorage,
		rand: &mut Xoroshiro64Star,
	) -> SpreadResult {
		if (self.chance / TPS as f32) >= rand.gen_range(0.0..1.0) as f32 {
			let mut spread = None;
			let mut keep = false;
			for dir in Direction::values() {
				if let Some(pos) = pos.checked_offset(dir.offset()) {
					if let Some(chunk) = chunks.get_mut(pos.chunk) {
						let layer = &mut chunk.layers[layer_id];
						let id = layer[pos.entry].id;
						if let Some(next_id) = self.convert_table.get(&id) {
							if spread.is_some() {
								keep = true;
								break;
							}

							spread = Some((pos, *next_id));
						}
					}
				}
			}

			// we could not spread in the 4 directions
			SpreadResult { keep, spread }
		} else {
			SpreadResult {
				keep: true,
				spread: None,
			}
		}
	}
}

pub struct SpreadResult {
	pub keep: bool,
	pub spread: Option<(BlockPos, Id<BlockType>)>,
}

#[derive(Debug, FromLua)]
pub struct BlockSpreaderPrototype {
	pub chance: f32,
	pub convert_table: HashMap<Identifier, Identifier>,
}

#[lua_impl]
impl BlockSpreaderPrototype {
	pub fn bake(self, blocks: &RegistryLookup<BlockType>) -> Result<BlockSpreader> {
		let mut convert_table = HashMap::new();
		for (from, to) in &self.convert_table {
			convert_table.insert(
				blocks
					.get_id(from)
					.wrap_err_with(|| format!("Could not find from target {}", from))?,
				blocks
					.get_id(to)
					.wrap_err_with(|| format!("Could not find to target {}", from))?,
			);
		}

		Ok(BlockSpreader {
			chance: self.chance,
			convert_table,
		})
	}
}