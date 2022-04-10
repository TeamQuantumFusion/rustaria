use std::str::FromStr;
use rustaria_api::tag::Tag;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, ChunkSubPos};
use crate::api::Api;
use crate::api::prototype::tile::TilePrototype;
use crate::world::chunk::{Chunk, ChunkLayer};
use rustaria_util::{ContextCompat, Result};

pub fn generate_chunk(api: &Api, pos: ChunkPos) -> Result<Chunk> {
	let instance = api.instance();
	let air = instance
		.get_registry::<TilePrototype>()
		.create_from_tag(&Tag::from_str("rustaria:air")?).wrap_err("Could not create tile")?;

	let dirt = instance
		.get_registry::<TilePrototype>()
		.create_from_tag(&Tag::from_str("rustaria:dirt")?).wrap_err("Could not create tile")?;

	let mut chunk = Chunk {
		tiles: ChunkLayer::new([[air; CHUNK_SIZE]; CHUNK_SIZE]),
	};


	for y in 0..CHUNK_SIZE {
		for x in 0..CHUNK_SIZE {
			if ((y + pos.y as usize) ^ (x + pos.x as usize)) % 4 == 0 {
				chunk.tiles.put(dirt, ChunkSubPos::new(x as u8, y as u8));
			}
		}
	}


	Ok(chunk)
}