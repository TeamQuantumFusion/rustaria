#![allow(unused)] // alpha, remove this when you're done - leocth

use crate::api::{Prototype, RustariaApi};
use crate::chunk::tile::Tile;
use crate::chunk::wall::Wall;
use crate::registry::{Id, Tag};
use crate::types::{ChunkSubPos, CHUNK_SIZE, Direction};

pub mod fluid;
pub mod foliage;
pub mod tile;
pub mod tree;
pub mod wall;

#[derive(Copy, Clone)]
pub struct Chunk {
    pub tiles: ChunkGrid<Tile, Neighbor>,
    pub walls: ChunkGrid<Wall, Neighbor>,
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

#[derive(Copy, Clone)]
pub struct ChunkGrid<V, N>
where
    V: Clone + Copy,
    N: NeighborType<V> + Clone + Copy,
{
    grid: [[V; CHUNK_SIZE]; CHUNK_SIZE],
    neighbor_matrix: [[N; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<V, N> ChunkGrid<V, N>
where
    V: Clone + Copy,
    N: NeighborType<V> + Clone + Copy,
{
    pub fn new(value: V) -> ChunkGrid<V, N> {
        ChunkGrid {
            grid: [[value; CHUNK_SIZE]; CHUNK_SIZE],
            neighbor_matrix: [[N::new(&value); CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    fn get(&self, pos: ChunkSubPos) -> &V {
        debug_assert!(
            pos.x() < CHUNK_SIZE as u8 && pos.y() < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );
        &self.grid[pos.y() as usize][pos.x() as usize]
    }

    fn set(&mut self, pos: ChunkSubPos, value: V) {
        debug_assert!(
            pos.x() < CHUNK_SIZE as u8 && pos.y() < CHUNK_SIZE as u8,
            "ChunkSubPos is too big."
        );

        self.grid[pos.y() as usize][pos.x() as usize] = value;
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
