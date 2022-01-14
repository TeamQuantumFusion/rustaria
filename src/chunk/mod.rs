#![allow(unused)] // alpha, remove this when you're done - leocth

use crate::api::{Prototype, RustariaApi};
use crate::chunk::tile::Tile;
use crate::chunk::wall::Wall;
use crate::registry::{Id, Tag};

pub mod fluid;
pub mod foliage;
pub mod tile;
pub mod tree;
pub mod wall;

pub const CHUNK_SIZE: usize = 24;

#[derive(Copy, Clone)]
pub struct Chunk {
    pub tiles: ChunkGrid<Tile>,
    pub walls: ChunkGrid<Wall>,
}

impl Chunk {
    pub fn new(api: &RustariaApi, default_tile: &Id, default_wall: &Id) -> Option<Chunk> {
        let tile = api.tiles.get_entry(default_tile)?;
        let wall = api.walls.get_entry(default_wall)?;
        Some(Chunk {
            tiles: ChunkGrid::new(tile.create(*default_tile)),
            walls: ChunkGrid::new(wall.create(*default_wall)),
        })
    }
}

pub struct ChunkSubPos {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone)]
pub struct ChunkGrid<V: Clone + Copy> {
    grid: [[V; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<V: Clone + Copy> ChunkGrid<V> {
    pub fn new(value: V) -> ChunkGrid<V> {
        ChunkGrid {
            grid: [[value; CHUNK_SIZE]; CHUNK_SIZE]
        }
    }

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
