//! The module containing the world of Rustaria.

use std::fmt::{Display, Formatter};

use crate::api::Rustaria;
use crate::chunk::Chunk;
use crate::comps::Comps;
use crate::types::ChunkPos;

pub struct World {
    // size is chunks x chunks
    chunk_size: (u32, u32),
    // 4x4 chunk grid look like this in the vec
    // y[x,x,x,x], y[x,x,x,x], y[x,x,x,x], y[x,x,x,x]
    chunks: Vec<Chunk>,
    pub comps: Comps,
}

impl World {
    pub fn new(chunk_size: (u32, u32), chunks: Vec<Chunk>) -> Result<World, WorldCreationError> {
        if chunk_size.0 as usize * chunk_size.1 as usize != chunks.len() {
            return Err(WorldCreationError::InvalidWorldSize);
        }

        Ok(Self {
            chunk_size,
            chunks,
            comps: Comps::new(),
        })
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(pos.get_raw_pos(self.chunk_size)?)
    }

    fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(pos.get_raw_pos(self.chunk_size)?)
    }

    pub fn tick(&mut self, rustaria: &Rustaria) {}
}

// NOTE(leocth): `thiserror` might be appropriate here
#[derive(Debug)]
pub enum WorldCreationError {
    InvalidWorldSize,
}

impl Display for WorldCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldCreationError::InvalidWorldSize => write!(
                f,
                "InvalidWorldSize: Chunk size does not match Chunk Storage."
            ),
        }
    }
}

impl std::error::Error for WorldCreationError {}

#[cfg(test)]
mod tests {
    use crate::chunk;
    use crate::types::ChunkPos;
    use crate::world::World;

    const WORLD_SIZE: usize = 10;

    #[test]
    fn roundtrip_chunk_set() {
        let world_size = (WORLD_SIZE as u32, WORLD_SIZE as u32);

        // Set
        let mut chunks = Vec::new();
        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                chunks.push(chunk::tests::new(x as u32, y as u32));
            }
        }

        let world = World::new(world_size, chunks.clone()).unwrap();

        // Get
        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let pos = ChunkPos {
                    x: x as u32,
                    y: y as u32,
                };
                assert_eq!(
                    world.get_chunk(pos).unwrap(),
                    &chunk::tests::new(x as u32, y as u32)
                );
            }
        }
    }
}
