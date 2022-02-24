use std::collections::HashMap;
use std::ops::BitXorAssign;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use std::thread;
use std::time::{Duration};

use crossbeam::channel::{Receiver, Sender};


use opengl_render::atlas::{Atlas, AtlasLocation};
use opengl_render::buffer::{Buffer, BufferAccess, BufferUsage};
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
    buffer: Buffer<QuadImageVertex>,
    pub index_buffer: Buffer<u32>,
    mesh_channel: (Sender<(Vec<QuadImageVertex>, Vec<u32>)>, Receiver<(Vec<QuadImageVertex>, Vec<u32>)>),

    tile_lookup: Vec<Option<RenderTile>>,
    chunks: Arc<RwLock<HashMap<ChunkPos, RenderChunk>>>,
    new_chunks: HashMap<ChunkPos, RenderChunk>,
}

impl WorldMeshHandler {
    pub fn new(rsa: &Rustaria, atlas: &Atlas<AtlasId>, buffer: Buffer<QuadImageVertex>, index_buffer: Buffer<u32>) -> eyre::Result<WorldMeshHandler> {
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
            buffer,
            index_buffer,
            mesh_channel: crossbeam::channel::unbounded(),
            tile_lookup: lookup,
            chunks: Default::default(),
            new_chunks: Default::default(),
        })
    }

    pub fn add_chunk(&mut self, pos: ChunkPos, chunk: &Chunk) {
        let mut render_chunk = RenderChunk::new(chunk, self);
        render_chunk.compile_internal();
        self.new_chunks.insert(pos, render_chunk);
    }

    fn compile_borders(chunks: &mut HashMap<ChunkPos, RenderChunk>, pos: ChunkPos, chunk: &mut RenderChunk) {
        for offset in Direction::all() {
            if let Some(neighbor_pos) = pos.offset(offset) {
                if let Some(neighbor) = chunks.get_mut(&neighbor_pos) {
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

    pub fn tick(&mut self) {
        if !self.new_chunks.is_empty() {
            if let Ok(mut chunks) = self.chunks.try_write() {
                for (pos, mut chunk) in self.new_chunks.drain() {
                    Self::compile_borders(&mut chunks, pos, &mut chunk);
                    chunks.insert(pos, chunk);
                }
                std::mem::drop(chunks);
                let channel = self.mesh_channel.0.clone();
                let chunks = self.chunks.clone();
                thread::spawn(move || {
                    if let Ok(read) = chunks.read() {
                        Self::build_mesh(channel, read);
                    }
                });
            }
        }



        if let Ok((vertex, index)) = self.mesh_channel.1.try_recv() {
            self.index_buffer.upload(&index, BufferUsage::Static, BufferAccess::Draw);
            self.buffer.upload(&vertex, BufferUsage::Static, BufferAccess::Draw);
        }
    }

    fn build_mesh(channel: Sender<(Vec<QuadImageVertex>, Vec<u32>)>, read: RwLockReadGuard<HashMap<ChunkPos, RenderChunk>>) {
        thread::sleep(Duration::from_millis(100));
        let mut data = Vec::new();
        let mut indices = Vec::new();

        for (pos, chunk) in read.iter() {
            for y in 0..CHUNK_SIZE {
                let row = &chunk.tile.grid[y];
                let row_neighbor = &chunk.tile_neighbor.grid[y];
                for x in 0..CHUNK_SIZE {
                    if let Some(tile) = &row[x] {
                        let img = tile.sprite;
                        let tile = &row_neighbor[x];

                        let x = x as f32 + (pos.x as f32 * CHUNK_SIZE as f32);
                        let y = y as f32 + (pos.y as f32 * CHUNK_SIZE as f32);

                        let var = {
                            let mut p = u32::from_le_bytes(x.to_be_bytes()).wrapping_mul(9).wrapping_add(u32::from_le_bytes(y.to_le_bytes()));
                            p.bitxor_assign(p.wrapping_shl(11));
                            p.bitxor_assign(p.wrapping_shr(7));
                            p.bitxor_assign(p.wrapping_shl(17));
                            p
                        } % 3;

                        let tile_size = img.height / 4.0;
                        let (o_x, o_y) = TileImagePos::new(tile.up, tile.dw, tile.lf, tile.rg).get_tex_pos();
                        let (t_x, t_y) = (img.x + (o_x * tile_size) + (var as f32 * img.height), img.y + (o_y * tile_size));
                        // todo custom variant amounts. currently its forced to be 3
                        // todo make a builder.
                        //0, 1, 3, 1, 2, 3u32
                        let len = data.len() as u32;
                        indices.push(0 + len);
                        indices.push(1 + len);
                        indices.push(3 + len);
                        indices.push(1 + len);
                        indices.push(2 + len);
                        indices.push(3 + len);
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

        channel.send((data, indices)).unwrap();
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