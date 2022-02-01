#![allow(unused)] // alpha, remove this when you're done - leocth

use crate::api::{Prototype, Rustaria};
use crate::chunk::tile::Tile;
use crate::chunk::wall::Wall;
use crate::registry::{RawId, Tag};
use crate::types::{ChunkSubPos, Direction, CHUNK_SIZE};

use self::tile::TilePrototype;
use self::wall::WallPrototype;

pub mod fluid;
pub mod foliage;
pub mod tile;
pub mod tree;
pub mod wall;

#[derive(Copy, Clone)]
pub struct Chunk {
    pub tiles: ChunkGrid<Tile>,
    pub walls: ChunkGrid<Wall>,
}

impl Chunk {
    pub fn new(api: &Rustaria, default_tile: RawId, default_wall: RawId) -> Option<Chunk> {
        let tile = api.tiles.get_from_id(default_tile)?;
        let wall = api.walls.get_from_id(default_wall)?;
        Some(Chunk {
            tiles: ChunkGrid::new(tile.create(default_tile)),
            walls: ChunkGrid::new(wall.create(default_wall)),
        })
    }
}

#[derive(Copy, Clone)]
pub struct ChunkGrid<V>
where
    V: Clone + Copy,
{
    pub grid: [[V; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<V> ChunkGrid<V>
where
    V: Clone + Copy,
{
    pub fn new(value: V) -> ChunkGrid<V> {
        ChunkGrid {
            grid: [[value; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn get(&self, pos: ChunkSubPos) -> &V {
        debug_assert!(
            pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );
        &self.grid[pos.y as usize][pos.x as usize]
    }

    pub fn set(&mut self, pos: ChunkSubPos, value: V) {
        debug_assert!(
            pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );
        self.grid[pos.y as usize][pos.x as usize] = value;
    }

    pub fn get_mut(&mut self, pos: ChunkSubPos) -> &mut V {
        debug_assert!(
            pos.x < CHUNK_SIZE as u8 && pos.y < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );

        &mut self.grid[pos.y as usize][pos.x as usize]
    }
}

#[derive(Copy, Clone)]
pub enum Neighbor {
    Solid,
    Air,
}

pub trait NeighborType<V>
where
    V: Clone + Copy,
{
    fn new(value: &V) -> Self;
}

impl NeighborType<Tile> for Neighbor {
    fn new(value: &Tile) -> Self {
        Neighbor::Solid
    }
}

impl NeighborType<Wall> for Neighbor {
    fn new(value: &Wall) -> Self {
        Neighbor::Solid
    }
}
