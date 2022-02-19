#![allow(unused)] // alpha, remove this when you're done - leocth


use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use rsa_common::types::{ChunkSubPos, RawId};
use crate::tile::Tile;
use crate::wall::Wall;

pub mod fluid;
pub mod foliage;
pub mod tile;
pub mod tree;
pub mod wall;

pub const CHUNK_SIZE: usize = 24;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: ChunkGrid<Tile>,
    pub walls: ChunkGrid<Wall>,
}

impl Chunk {
    pub fn new(default_tile: Tile, default_wall: Wall) -> Option<Chunk> {
        Some(Chunk {
            tiles: ChunkGrid::new(default_tile),
            walls: ChunkGrid::new(default_wall),
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
pub mod tests {
    use rsa_common::types::RawId;
    use crate::{Chunk, CHUNK_SIZE, ChunkGrid};

    pub fn new(tile_id: RawId, wall_id: RawId) -> Chunk {
        Chunk {
            tiles: ChunkGrid {
                grid: [[crate::tile::tests::new(tile_id); CHUNK_SIZE]; CHUNK_SIZE],
            },
            walls: ChunkGrid {
                grid: [[crate::wall::tests::new(wall_id); CHUNK_SIZE]; CHUNK_SIZE],
            },
        }
    }
}
