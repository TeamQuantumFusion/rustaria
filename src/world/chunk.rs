use serde::{Deserialize, Serialize};

use rustaria_util::ty::{CHUNK_SIZE, ChunkSubPos};

use crate::world::tile::Tile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub tiles: ChunkLayer<Tile>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkLayer<T> {
   pub grid: [[T; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<T> ChunkLayer<T> {
    pub fn new(values: [[T; CHUNK_SIZE]; CHUNK_SIZE]) -> ChunkLayer<T> {
        ChunkLayer {
            grid: values,
        }
    }

    #[inline(always)]
    pub fn get(&self, pos: ChunkSubPos) -> &T {
        &self.grid[pos.y() as usize][pos.x() as usize]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: ChunkSubPos) -> &mut T {
        &mut self.grid[pos.y() as usize][pos.x() as usize]
    }

    #[inline(always)]
    pub fn put(&mut self, value: T, pos: ChunkSubPos) {
        self.grid[pos.y() as usize][pos.x() as usize] = value;
    }
}

