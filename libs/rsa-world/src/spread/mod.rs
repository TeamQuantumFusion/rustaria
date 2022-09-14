pub mod block;

use std::collections::HashMap;

use rand::SeedableRng;
use rand_xoshiro::Xoroshiro64Star;
use rsa_core::{
	debug::{DebugCategory, DebugRendererImpl},
	draw_debug,
};
use rsa_registry::Id;

use crate::{BlockPos, ChunkLayerType, ChunkStorage, rpc::WorldAPI};
use crate::chunk::block::ty::BlockType;

pub struct SpreaderSystem {
	rand: Xoroshiro64Star,
	active_spreads: HashMap<(BlockPos, Id<ChunkLayerType>), Id<BlockType>>,
}

impl SpreaderSystem {
	pub fn new() -> SpreaderSystem {
		SpreaderSystem {
			rand: Xoroshiro64Star::seed_from_u64(69420),
			active_spreads: Default::default(),
		}
	}

	pub fn tick(
		&mut self,
		rpc: &WorldAPI,
		chunks: &mut ChunkStorage,
		debug: &mut impl DebugRendererImpl,
	) -> Vec<(BlockPos, Id<ChunkLayerType>, Id<BlockType>)> {
		// Spread
		let mut remove = Vec::new();
		let mut spread = Vec::new();
		for ((pos, layer_id), block_id) in &self.active_spreads {
			let prototype = &rpc.block_layer[*layer_id].blocks[*block_id];
			if let Some(prototype) = &prototype.spread {
				let result = prototype.tick_spread(*pos, *layer_id, chunks, &mut self.rand);
				if let Some(result) = result.spread {
					draw_debug!(
						debug,
						DebugCategory::TileSpread,
						result.0,
						0xfcfcfa,
						10.0,
						1.0
					);
					spread.push((result.0, *layer_id, result.1));
				}

				if !result.keep {
					draw_debug!(debug, DebugCategory::TileSpread, *pos, 0xbf5570, 1.0, 1.0);
					remove.push((*pos, *layer_id));
				} else {
					draw_debug!(debug, DebugCategory::TileSpread, *pos, 0x5b595c);
				}
			}
		}

		for pos in remove {
			self.active_spreads.remove(&pos);
		}

		spread
	}

	pub fn place_block(
		&mut self,
		pos: BlockPos,
		layer_id: Id<ChunkLayerType>,
		block_id: Id<BlockType>,
		desc: &BlockType,
	) {
		self.active_spreads.remove(&(pos, layer_id));
		if desc.spread.is_some() {
			self.active_spreads.insert((pos, layer_id), block_id);
		}
	}
}
