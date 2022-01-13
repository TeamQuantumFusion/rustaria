#![allow(unused)] // alpha, remove this when you're done - leocth

use crate::chunk::tile::{Tile};
use crate::chunk::wall::Wall;

pub mod fluid;
pub mod foliage;
pub mod tile;
pub mod tree;
pub mod wall;

pub const CHUNK_SIZE: usize = 24;

pub struct Chunk {
    pub tiles: ChunkGrid<Tile>,
    pub walls: ChunkGrid<Wall>,
}

pub struct ChunkSubPos {
    x: u8,
    y: u8,
}

pub struct ChunkGrid<V> {
    grid: [[V; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<V> ChunkGrid<V> {
    fn get(&self, pos: ChunkSubPos) -> &V {
        debug_assert!(
            pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );
        &self.grid[pos.y as usize][pos.x as usize]
    }

    fn set(&mut self, pos: ChunkSubPos, value: V) {
        debug_assert!(
            pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );
        self.grid[pos.y as usize][pos.x as usize] = value;
    }
}
