use std::collections::HashMap;
use aloy::atlas::Atlas;
use rustaria::api::Api;
use rustaria::api::prototype::tile::TilePrototype;
use rustaria::api::ty::ConnectionType;
use rustaria::world::chunk::{Chunk, ChunkLayer};
use rustaria_api::tag::Tag;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, ChunkSubPos, Direction, Offset};

use crate::{Pos, VertexBuilder};
use crate::renderer::atlas::TextureAtlas;
use crate::ty::Texture;
use crate::world_drawer::tile::BakedTile;

pub struct BakedChunk {
    pub tiles: ChunkLayer<Option<BakedTile>>,
    pub tile_neighbors: ChunkLayer<NeighborMatrix>,
}

impl BakedChunk {
    pub fn new(api: &Api, chunk: &Chunk, atlas: &TextureAtlas) -> BakedChunk {
        let instance = api.instance();
        let registry = instance.get_registry::<TilePrototype>();
        let mut tiles = ChunkLayer::new([[None; CHUNK_SIZE]; CHUNK_SIZE]);
        let mut tile_neighbors = ChunkLayer::new([[EMPTY_MATRIX; CHUNK_SIZE]; CHUNK_SIZE]);

        for y in 0..CHUNK_SIZE {
            let baked_row = &mut tiles.grid[y];
            let row = &chunk.tiles.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = BakedTile::new(registry, &row[x], atlas) {
                    baked_row[x] = Some(tile);
                }
            }
        }

        BakedChunk {
            tiles,
            tile_neighbors,
        }
    }

    pub fn compile_internal(&mut self) {
        for y in 0..CHUNK_SIZE {
            let row = &self.tiles.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &row[x] {
                    if tile.ty == ConnectionType::Connected {
                        if y != CHUNK_SIZE - 1 {
                            if let Some(top_tile) = &self.tiles.grid[y + 1][x] {
                                if let ConnectionType::Connected = top_tile.ty {
                                    self.tile_neighbors.grid[y][x].up = ConnectionType::Connected;
                                    self.tile_neighbors.grid[y + 1][x].down = ConnectionType::Connected;
                                }
                            }
                        }

                        if x != CHUNK_SIZE - 1 {
                            if let Some(right_tile) = &row[x + 1] {
                                if let ConnectionType::Connected = right_tile.ty {
                                    self.tile_neighbors.grid[y][x].right = ConnectionType::Connected;
                                    self.tile_neighbors.grid[y][x + 1].left = ConnectionType::Connected;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn compile_chunk_borders(&mut self, chunks: &mut HashMap<ChunkPos, BakedChunk>, pos: ChunkPos) {
        for offset in Direction::all() {
            if let Some(neighbor_pos) = pos.offset(offset.into()) {
                if let Some(neighbor) = chunks.get_mut(&neighbor_pos) {
                    let y_offset = offset.offset_y().max(0) as usize * (CHUNK_SIZE - 1);
                    let x_offset = offset.offset_x().max(0) as usize * (CHUNK_SIZE - 1);
                    let y_length = (CHUNK_SIZE - 1) * (offset.offset_x().abs() as usize);
                    let x_length = (CHUNK_SIZE - 1) * (offset.offset_y().abs() as usize);
                    for y in y_offset..=y_length + y_offset {
                        let row = &self.tiles.grid[y];
                        // clippy having a stroke
                        #[allow(clippy::needless_range_loop)]
                        for x in x_offset..=x_length + x_offset {
                            let neighbor_sub_pos =
                                ChunkSubPos::new(x as u8, y as u8).euclid_offset(offset.into());

                            let mut ty = ConnectionType::Isolated;
                            if let Some(tile) = &row[x] {
                                if let Some(neighbor_tile) = neighbor.tiles.get(neighbor_sub_pos) {
                                    if let (ConnectionType::Connected, ConnectionType::Connected) =
                                    (tile.ty, neighbor_tile.ty)
                                    {
                                        ty = ConnectionType::Connected;
                                    }
                                }
                            }

                            self.tile_neighbors.grid[y][x].set(offset, ty);
                            neighbor
                                .tile_neighbors
                                .get_mut(neighbor_sub_pos)
                                .set(offset.flip(), ty);
                        }
                    }
                }
            }
        }
    }

    pub fn push(&self, builder: &mut VertexBuilder<(Pos, Texture)>, pos: ChunkPos) {
        for y in 0..CHUNK_SIZE {
            let tile_row = &self.tiles.grid[y];
            let tile_neighbor_row = &self.tile_neighbors.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &tile_row[x] {
                    tile.push(
                        &tile_neighbor_row[x],
                        builder,
                        (
                            (pos.x as f32 * CHUNK_SIZE as f32) + (x as f32),
                            (pos.y as f32 * CHUNK_SIZE as f32) + (y as f32),
                        ),
                    );
                }
            }
        }
    }
}

pub const EMPTY_MATRIX: NeighborMatrix = NeighborMatrix {
    up: ConnectionType::Isolated,
    down: ConnectionType::Isolated,
    left: ConnectionType::Isolated,
    right: ConnectionType::Isolated,
};

#[derive(Copy, Clone)]
pub struct NeighborMatrix {
    pub up: ConnectionType,
    pub down: ConnectionType,
    pub left: ConnectionType,
    pub right: ConnectionType,
}


impl NeighborMatrix {
    pub fn set(&mut self, dir: Direction, ty: ConnectionType) {
        match dir {
            Direction::Up => self.up = ty,
            Direction::Left => self.left = ty,
            Direction::Down => self.down = ty,
            Direction::Right => self.right = ty,
        }
    }
}