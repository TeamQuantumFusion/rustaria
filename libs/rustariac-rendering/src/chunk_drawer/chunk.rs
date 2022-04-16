use std::collections::HashMap;

use rustaria::api::{prototype::tile::TilePrototype, ty::ConnectionType};
use rustaria::chunk::{Chunk, ChunkLayer};
use rustaria_api::Carrier;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, ChunkSubPos, Direction, Offset};
use rustariac_backend::{builder::VertexBuilder, ClientBackend, ty::PosTexture};

use super::tile::{BakedTile, TileDrawer};

pub struct BakedChunk {
    pub tile_drawers: Vec<Option<TileDrawer>>,
    pub tiles: ChunkLayer<Option<BakedTile>>,
    pub tile_neighbors: ChunkLayer<NeighborMatrix>,
}

impl BakedChunk {
    pub fn new(carrier: &Carrier, chunk: &Chunk, backend: &ClientBackend) -> BakedChunk {
        let instance = carrier.lock();
        let registry = instance.get_registry::<TilePrototype>();
        let mut tiles = ChunkLayer::new([[None; CHUNK_SIZE]; CHUNK_SIZE]);
        let tile_neighbors = ChunkLayer::new([[EMPTY_MATRIX; CHUNK_SIZE]; CHUNK_SIZE]);
        for y in 0..CHUNK_SIZE {
            let baked_row = &mut tiles.grid[y];
            let row = &chunk.tiles.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = BakedTile::new(registry, &row[x]) {
                    baked_row[x] = Some(tile);
                }
            }
        }

        let mut tile_drawers = Vec::new();
        for prototype in registry.iter() {
            tile_drawers.push(TileDrawer::new(prototype, backend));
        }

        BakedChunk {
            tiles,
            tile_neighbors,
            tile_drawers,
        }
    }

    pub fn compile_internal(&mut self) {
        for y in 0..CHUNK_SIZE {
            let row = &self.tiles.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &row[x] {
                    if tile.connection == ConnectionType::Connected {
                        if y != CHUNK_SIZE - 1 {
                            if let Some(top_tile) = &self.tiles.grid[y + 1][x] {
                                if let ConnectionType::Connected = top_tile.connection {
                                    self.tile_neighbors.grid[y][x].up = ConnectionType::Connected;
                                    self.tile_neighbors.grid[y + 1][x].down =
                                        ConnectionType::Connected;
                                }
                            }
                        }

                        if x != CHUNK_SIZE - 1 {
                            if let Some(right_tile) = &row[x + 1] {
                                if let ConnectionType::Connected = right_tile.connection {
                                    self.tile_neighbors.grid[y][x].right =
                                        ConnectionType::Connected;
                                    self.tile_neighbors.grid[y][x + 1].left =
                                        ConnectionType::Connected;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn compile_chunk_borders(
        &mut self,
        chunks: &mut HashMap<ChunkPos, BakedChunk>,
        pos: ChunkPos,
    ) {
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
                                        (tile.connection, neighbor_tile.connection)
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

    pub fn push(&self, builder: &mut VertexBuilder<PosTexture>, tile_drawers: &Vec<Option<TileDrawer>>, pos: &ChunkPos) {
        for y in 0..CHUNK_SIZE {
            let tile_row = &self.tiles.grid[y];
            let tile_neighbor_row = &self.tile_neighbors.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &tile_row[x] {
                    if let Some(drawer) = &tile_drawers[tile.id.index()] {
                        let matrix = &tile_neighbor_row[x];
                        drawer.push(
                            builder,
                            (pos.x * CHUNK_SIZE as u32) + (x as u32),
                            (pos.y * CHUNK_SIZE as u32) + (y as u32),
                            super::tile::TileConnectionKind::new(
                                matrix.up,
                                matrix.down,
                                matrix.left,
                                matrix.right,
                            ),
                        );
                    }
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
