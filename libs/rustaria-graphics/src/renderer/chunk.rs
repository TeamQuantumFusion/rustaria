use aloy::atlas::Atlas;
use rustaria::api::Api;
use rustaria::api::prototype::tile::TilePrototype;
use rustaria::api::ty::ConnectionType;
use rustaria::world::chunk::{Chunk, ChunkLayer};
use rustaria_api::tag::Tag;
use rustaria_util::info;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, Direction};

use crate::{Pos, VertexBuilder};
use crate::ty::Texture;
use crate::renderer::tile::BakedTile;

pub struct BakedChunk {
    pub tiles: ChunkLayer<Option<BakedTile>>,
    pub tile_neighbors: ChunkLayer<NeighborMatrix>,
}

impl BakedChunk {
    pub fn new(api: &Api, chunk: &Chunk, atlas: &Atlas<Tag>) -> BakedChunk {
        let registry = api.get_registry::<TilePrototype>();
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

        for y in 0..CHUNK_SIZE {
            let row = &tiles.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &row[x] {
                    if tile.ty == ConnectionType::Connected {
                        if y != CHUNK_SIZE - 1 {
                            if let Some(top_tile) = &tiles.grid[y + 1][x] {
                                if let ConnectionType::Connected = top_tile.ty {
                                    tile_neighbors.grid[y][x].up = ConnectionType::Connected;
                                    tile_neighbors.grid[y + 1][x].down = ConnectionType::Connected;
                                }
                            }
                        }

                        if x != CHUNK_SIZE - 1 {
                            if let Some(right_tile) = &row[x + 1] {
                                if let ConnectionType::Connected = right_tile.ty {
                                    tile_neighbors.grid[y][x].right = ConnectionType::Connected;
                                    tile_neighbors.grid[y][x + 1].left = ConnectionType::Connected;
                                }
                            }
                        }
                    }
                }
            }
        }

        BakedChunk {
            tiles,
            tile_neighbors,
        }
    }

    pub fn push(&self, builder: &mut VertexBuilder<(Pos, Texture)>, pos: ChunkPos) {
        info!("Chunk {:?}", pos);
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
    down: ConnectionType::Connected,
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
            Direction::Top => self.up = ty,
            Direction::Left => self.left = ty,
            Direction::Bottom => self.down = ty,
            Direction::Right => self.right = ty,
        }
    }
}