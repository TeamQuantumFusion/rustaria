use eyre::{Result, ContextCompat};
use rustaria_api::ty::{Prototype, Tag};
use rustaria_api::Carrier;
use rustaria_util::ty::{ChunkPos, ChunkSubPos, CHUNK_SIZE};

use crate::api::prototype::tile::TilePrototype;
use crate::world::chunk::{Chunk, ChunkLayer};

pub fn generate_chunk(stack: &Carrier, pos: ChunkPos) -> Result<Chunk> {
    let instance = stack.lock();
    let tiles = instance.get_registry::<TilePrototype>();

    // We do a touch of unwrapping.
    let id = tiles.get_id(&Tag::new("rustaria:air".to_string())?).wrap_err("lol")?;
    let air = tiles.get_prototype(id).unwrap().create(id);

    let id = tiles.get_id(&Tag::new("rustaria:dirt".to_string())?).wrap_err("lol")?;
    let dirt = tiles.get_prototype(id).wrap_err("lmao")?.create(id);

    let mut chunk = Chunk {
        tiles: ChunkLayer::new([[air; CHUNK_SIZE]; CHUNK_SIZE]),
    };

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            if ((y + (pos.y as usize * CHUNK_SIZE)) ^ (x + (pos.x as usize * CHUNK_SIZE))) % 5 == 0
            {
                chunk.tiles.put(dirt, ChunkSubPos::new(x as u8, y as u8));
            }
        }
    }

    Ok(chunk)
}
