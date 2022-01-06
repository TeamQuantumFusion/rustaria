use crate::chunk::tile::Tile;
use crate::chunk::wall::Wall;

mod foliage;
mod tile;
mod tree;
mod wall;
mod fluid;

pub const CHUNK_SIZE: usize = 24;

pub struct Chunk {
    tiles: ChunkGrid<Tile>,
    walls: ChunkGrid<Wall>,
}

pub struct ChunkSubPos {
    x: u8,
    y: u8,
}

impl Chunk {
    pub fn get_tile(&self, pos: ChunkSubPos) -> &Tile {
        self.tiles.get(pos)
    }

    pub fn get_wall(&self, pos: ChunkSubPos) -> &Wall {
        self.walls.get(pos)
    }
}

struct ChunkGrid<V> {
    grid: [[V; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<V> ChunkGrid<V> {
    fn get(&self, pos: ChunkSubPos) -> &V {
        debug_assert!(pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8, "ChunkSubPos is too big.");
        &self.grid[pos.y as usize][pos.x  as usize]
    }
}