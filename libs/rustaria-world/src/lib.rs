use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use hecs::{DynamicBundle, Entity};

use rustaria_util::ty::ChunkPos;

use crate::chunk::Chunk;

mod chunk;

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    entities: hecs::World,
}

impl World {
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }
}

impl Deref for World {
    type Target = hecs::World;

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}
