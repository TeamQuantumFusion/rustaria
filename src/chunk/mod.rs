use crate::chunk::tile::Tile;
use crate::chunk::wall::Wall;

mod foliage;
mod tile;
mod tree;
mod wall;
mod water;

pub const CHUNK_SIZE: usize = 24;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    walls: [[Wall; CHUNK_SIZE]; CHUNK_SIZE],
}

pub struct ChunkSubPos {
    x: u8,
    y: u8,
}

impl Chunk {
    pub fn get_tile(&self, pos: ChunkSubPos) -> &Tile {
        debug_assert!(pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8, "ChunkSubPos is too big.");
        &self.tiles[pos.y as usize][pos.x as usize]
    }

    pub fn get_wall(&self, pos: ChunkSubPos) -> &Wall {
        debug_assert!(pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8, "ChunkSubPos is too big.");
        &self.walls[pos.y as usize][pos.x as usize]
    }
}