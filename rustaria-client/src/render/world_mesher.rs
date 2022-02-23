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
            if prot.sprite.is_some() {
                let sprite = atlas.lookup.get(&AtlasId::Tile(id as u32)).unwrap_or_else(|| atlas.lookup.get(&AtlasId::Missing).unwrap());
                lookup.push(Option::Some(RenderTile {
                    ty: prot.connection,
                    sprite: *sprite,
                }))
            } else {
                lookup.push(Option::None)
            }
        }
        Ok(WorldMeshHandler {
            tile_lookup: lookup,
            chunks: Default::default(),
        })
    }

    pub fn add_chunk(&mut self, pos: ChunkPos, chunk: &Chunk) {
        let start = Instant::now();
        let mut render_chunk = RenderChunk::new(chunk, self);

        render_chunk.compile_internal();
        self.compile_borders(pos, &mut render_chunk);

        info!("Compiled chunk borders in {}ms", start.elapsed().as_micros() as f32 / 1000.0);
        self.chunks.insert(pos, render_chunk);
    }

    fn compile_borders(&mut self, pos: ChunkPos, chunk: &mut RenderChunk) {
        for offset in Direction::all() {
            if let Some(neighbor_pos) = pos.offset(offset) {
                if let Some(neighbor) = self.chunks.get_mut(&neighbor_pos) {
                    let pos = ChunkSubPos {
                        x: ((CHUNK_SIZE - 1) * (offset.offset_y().abs() as usize)) as u8,
                        y: ((CHUNK_SIZE - 1) * (offset.offset_x().abs() as usize)) as u8,
                    };

                    let y_offset = offset.offset_y().max(0) as u8 * (CHUNK_SIZE as u8 - 1);
                    let x_offset = offset.offset_x().max(0) as u8 * (CHUNK_SIZE as u8 - 1);
                    for y in 0..=pos.y {
                        let y = y + y_offset;
                        let row = &chunk.tile.grid[y as usize];
                        for x in 0..=pos.x {
                            let x = x + x_offset;
                            let neighbor_sub_pos = ChunkSubPos { x, y }.overflowing_offset(offset);
                            let neighbor_tile = neighbor.tile.get(neighbor_sub_pos);
                            if let Some(tile) = &row[x as usize] {
                                if let Some(neighbor_tile) = neighbor_tile {
                                    if let (ConnectionType::Connected, ConnectionType::Connected) = (tile.ty, neighbor_tile.ty) {
                                        chunk.tile_neighbor.grid[y as usize][x as usize].set(offset, ConnectionType::Connected);
                                        neighbor.tile_neighbor.get_mut(neighbor_sub_pos).set(offset.flip(), ConnectionType::Connected);
                                        // get out of here else isolated will run
                                        continue;
                                    }
                                }
                            }

                            // if fail
                            chunk.tile_neighbor.grid[y as usize][x as usize].set(offset, ConnectionType::Isolated);
                            neighbor.tile_neighbor.get_mut(neighbor_sub_pos).set(offset.flip(), ConnectionType::Isolated);
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
        // compile to every top right neighbor
        for y in 0..CHUNK_SIZE {
            let row = &self.tile.grid[y];
            // pretty lazy approach to not get index out of bounds but whatever.
            let top_row = if y != CHUNK_SIZE - 1 {
                &self.tile.grid[y + 1]
            } else {
                row
            };
            for x in 0..CHUNK_SIZE {
                if let Some(tile) = &row[x] {
                    if tile.ty == ConnectionType::Connected {
                        // check if its on the top iteration, where the top neighbor is in another chunk.
                        if y != CHUNK_SIZE - 1 {
                            if let Some(top_tile) = &top_row[x] {
                                if let ConnectionType::Connected = top_tile.ty {
                                    self.tile_neighbor.grid[y][x].up = ConnectionType::Connected;
                                    self.tile_neighbor.grid[y + 1][x].dw = ConnectionType::Connected;
                                }
                            }
                        }

                        // check if its on the right most iteration, where the right neighbor is in another chunk.
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