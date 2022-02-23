use std::collections::HashMap;
use std::time::Instant;

use tracing::info;

use opengl_render::atlas::{Atlas, AtlasLocation};
use rustaria::api::Rustaria;
use rustaria::api::types::ConnectionType;
use rustaria::chunk::{Chunk, ChunkGrid};
use rustaria::types::{CHUNK_SIZE, ChunkPos, ChunkSubPos, Direction, Offset};

use crate::render::texture_format::TileImagePos;
use crate::render::world_render::{AtlasId, QuadImageVertex};

const EMPTY_MATRIX: NeighborMatrix = NeighborMatrix {
    up: ConnectionType::Isolated,
    dw: ConnectionType::Isolated,
    lf: ConnectionType::Isolated,
    rg: ConnectionType::Isolated,
};

pub struct RenderChunk {
    tile_neighbor: ChunkGrid<NeighborMatrix>,
    tile: ChunkGrid<Option<RenderTile>>,
}

#[derive(Copy, Clone)]
pub struct NeighborMatrix {
    up: ConnectionType,
    dw: ConnectionType,
    lf: ConnectionType,
    rg: ConnectionType,
}

impl NeighborMatrix {
    pub fn set(&mut self, dir: Direction, ty: ConnectionType) {
        match dir {
            Direction::Top => self.up = ty,
            Direction::Left => self.lf = ty,
            Direction::Bottom => self.dw = ty,
            Direction::Right => self.rg = ty,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RenderTile {
    ty: ConnectionType,
    sprite: AtlasLocation,
}

pub struct WorldMeshHandler {
    tile_lookup: Vec<Option<RenderTile>>,
    chunks: HashMap<ChunkPos, RenderChunk>,
}

impl WorldMeshHandler {
    pub fn new(rsa: &Rustaria, atlas: &Atlas<AtlasId>) -> eyre::Result<WorldMeshHandler> {
        let mut lookup = Vec::new();
        for (id, prot) in rsa.tiles.entries().iter().enumerate() {
            lookup.push(prot.sprite.is_some().then(||
                RenderTile {
                    ty: prot.connection,
                    sprite: *atlas.lookup.get(&AtlasId::Tile(id as u32)).unwrap_or_else(|| atlas.lookup.get(&AtlasId::Missing).unwrap()),
                }
            ));
        }
        Ok(WorldMeshHandler {
            tile_lookup: lookup,
            chunks: Default::default(),
        })
    }

    pub fn add_chunk(&mut self, pos: ChunkPos, chunk: &Chunk) {
        let mut render_chunk = RenderChunk::new(chunk, self);

        render_chunk.compile_internal();
        self.compile_borders(pos, &mut render_chunk);

        self.chunks.insert(pos, render_chunk);
    }

    fn compile_borders(&mut self, pos: ChunkPos, chunk: &mut RenderChunk) {
        for offset in Direction::all() {
            if let Some(neighbor_pos) = pos.offset(offset) {
                if let Some(neighbor) = self.chunks.get_mut(&neighbor_pos) {
                    let y_offset = offset.offset_y().max(0) as usize * (CHUNK_SIZE - 1);
                    let x_offset = offset.offset_x().max(0) as usize * (CHUNK_SIZE - 1);
                    let y_length = (CHUNK_SIZE - 1) * (offset.offset_x().abs() as usize);
                    let x_length = (CHUNK_SIZE - 1) * (offset.offset_y().abs() as usize);
                    for y in y_offset..=y_length + y_offset {
                        let row = &chunk.tile.grid[y];
                        // clippy having a stroke
                        #[allow(clippy::needless_range_loop)]
                        for x in x_offset..=x_length + x_offset {
                            let neighbor_sub_pos = ChunkSubPos { x: x as u8, y: y as u8 }.overflowing_offset(offset);

                            let mut ty = ConnectionType::Isolated;
                            if let Some(tile) = &row[x] {
                                if let Some(neighbor_tile) = neighbor.tile.get(neighbor_sub_pos) {
                                    if let (ConnectionType::Connected, ConnectionType::Connected) = (tile.ty, neighbor_tile.ty) {
                                        ty = ConnectionType::Connected;
                                    }
                                }
                            }

                            chunk.tile_neighbor.grid[y][x].set(offset, ty);
                            neighbor.tile_neighbor.get_mut(neighbor_sub_pos).set(offset.flip(), ty);
                        }
                    }
                }
            }
        }
    }

    pub fn build(&self) -> Vec<QuadImageVertex> {
        let mut data = Vec::new();

        for (pos, chunk) in &self.chunks {
            for y in 0..CHUNK_SIZE {
                let row = &chunk.tile.grid[y];
                let row_neighbor = &chunk.tile_neighbor.grid[y];
                for x in 0..CHUNK_SIZE {
                    if let Some(tile) = &row[x] {
                        let img = tile.sprite;
                        let tile = &row_neighbor[x];

                        let x = x as f32 + (pos.x as f32 * CHUNK_SIZE as f32);
                        let y = y as f32 + (pos.y as f32 * CHUNK_SIZE as f32);
                        let tile_size = img.height / 4.0;
                        let (o_x, o_y) = TileImagePos::new(tile.up, tile.dw, tile.lf, tile.rg).get_tex_pos();
                        let (t_x, t_y) = (img.x + (o_x * tile_size), img.y + (o_y * tile_size));
                        // todo custom variant amounts. currently its forced to be 3
                        // todo make a builder.
                        data.push(QuadImageVertex {
                            pos: [(x + 1.0), (y + 1.0)],
                            pos_texture: [t_x + tile_size, t_y],
                        });
                        data.push(QuadImageVertex {
                            pos: [(x + 1.0), (y + 0.0)],
                            pos_texture: [t_x + tile_size, t_y + tile_size],
                        });
                        data.push(QuadImageVertex {
                            pos: [(x + 0.0), (y + 0.0)],
                            pos_texture: [t_x, t_y + tile_size],
                        });
                        data.push(QuadImageVertex {
                            pos: [(x + 0.0), (y + 1.0)],
                            pos_texture: [t_x, t_y],
                        });
                    }
                }
            }
        }

        data
    }
}


impl RenderChunk {
    pub fn new(chunk: &Chunk, handler: &WorldMeshHandler) -> RenderChunk {
        let mut tile = ChunkGrid::new(None);
        for (y, y_t) in chunk.tiles.grid.iter().enumerate() {
            for (x, x_t) in y_t.iter().enumerate() {
                if let Some(Some(render_tile)) = handler.tile_lookup.get(x_t.id as usize) {
                    tile.grid[y][x] = Some(*render_tile);
                }
            }
        }

        RenderChunk {
            tile_neighbor: ChunkGrid::new(EMPTY_MATRIX),
            tile,
        }
    }

    fn compile_internal(&mut self) {
        for y in 0..CHUNK_SIZE {
            let row = &self.tile.grid[y];
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &row[x] {
                    if tile.ty == ConnectionType::Connected {
                        if y != CHUNK_SIZE - 1 {
                            if let Some(top_tile) = &self.tile.grid[y + 1][x] {
                                if let ConnectionType::Connected = top_tile.ty {
                                    self.tile_neighbor.grid[y][x].up = ConnectionType::Connected;
                                    self.tile_neighbor.grid[y + 1][x].dw = ConnectionType::Connected;
                                }
                            }
                        }

                        if x != CHUNK_SIZE - 1 {
                            if let Some(right_tile) = &row[x + 1] {
                                if let ConnectionType::Connected = right_tile.ty {
                                    self.tile_neighbor.grid[y][x].rg = ConnectionType::Connected;
                                    self.tile_neighbor.grid[y][x + 1].lf = ConnectionType::Connected;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}