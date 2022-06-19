use std::collections::HashMap;

use eyre::ContextCompat;
use apollo::{FromLua, Lua, Value};
use rand::Rng;
use rand_xoshiro::Xoroshiro64Star;

use crate::{
	api::util::lua_table,
	ty::{block_pos::BlockPos, direction::Direction, id::Id, identifier::Identifier, Offset},
	world::chunk::{block::BlockDesc, layer::BlockLayer},
	ChunkStorage, TPS,
};

pub struct BlockSpreader {
	pub chance:        f32,
	pub convert_table: HashMap<Id<BlockDesc>, Id<BlockDesc>>,
}

impl BlockSpreader {
	pub fn tick_spread(
		&self,
		pos: BlockPos,
		layer_id: Id<BlockLayer>,
		chunks: &mut ChunkStorage,
		rand: &mut Xoroshiro64Star,
	) -> SpreadResult {
		if (self.chance / TPS as f32) >= rand.gen_range(0.0..1.0) as f32 {
			let mut spread = None;
			let mut keep = false;
			for dir in Direction::values() {
				if let Some(pos) = pos.checked_offset(dir.offset()) {
					if let Some(chunk) = chunks.get_mut(pos.chunk) {
						let layer = chunk.layers.get_mut(layer_id);
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
				keep:   true,
				spread: None,
			}
		}
	}
}

pub struct SpreadResult {
	pub keep:   bool,
	pub spread: Option<(BlockPos, Id<BlockDesc>)>,
}

#[derive(Debug)]
pub struct BlockSpreaderPrototype {
	pub chance:        f32,
	pub convert_table: HashMap<Identifier, Identifier>,
}

impl BlockSpreaderPrototype {
	pub fn bake(self, blocks: &HashMap<Identifier, Id<BlockDesc>>) -> eyre::Result<BlockSpreader> {
		let mut convert_table = HashMap::new();
		for (from, to) in &self.convert_table {
			convert_table.insert(
				*blocks
					.get(from)
					.wrap_err(format!("Could not find from target {}", from))?,
				*blocks
					.get(to)
					.wrap_err(format!("Could not find to target {}", to))?,
			);
		}

		Ok(BlockSpreader {
			chance: self.chance,
			convert_table,
		})
	}
}

impl FromLua for BlockSpreaderPrototype {
	fn from_lua(lua_value: Value, _: &Lua) -> eyre::Result<Self> {
		let table = lua_table(lua_value)?;
		Ok(BlockSpreaderPrototype {
			chance:        table.get("chance")?,
			convert_table: table.get("convert_table")?,
		})
	}
}
