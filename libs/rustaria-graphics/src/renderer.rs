use std::collections::HashMap;
use std::str::FromStr;

use image::{DynamicImage, ImageFormat, ImageResult};

use aloy::atlas::{Atlas, AtlasBuilder};
use aloy::attribute::{AttributeDescriptor, AttributeType};
use rustaria::api::prototype::tile::TilePrototype;
use rustaria::api::Api;
use rustaria::api::ty::ConnectionType;
use rustaria::world::chunk::Chunk;
use rustaria_api::plugin::archive::ArchivePath;
use rustaria_api::tag::Tag;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos, ChunkSubPos, Direction, Offset};
use rustaria_util::{eyre, info, Result, warn, WrapErr};

use crate::renderer::chunk::BakedChunk;
use crate::ty::{Color, Pos, Texture, Player};
use crate::{DrawPipeline, Profiler, RenderLayerStability, VertexBuilder};

mod chunk;
mod tile;
pub mod pipeline;

pub struct WorldRenderer {
   pub atlas: Atlas<Tag>,
   pub color: DrawPipeline<(Pos, Color)>,
   pub texture: DrawPipeline<(Pos, Texture)>,
    chunks: HashMap<ChunkPos, BakedChunk>,
    world_dirty: bool,
}

impl WorldRenderer {
    pub fn new(api: &Api) -> WorldRenderer {
        let mut atlas_builder = AtlasBuilder::new();
        for prototype in api.get_registry::<TilePrototype>().entries() {
            if let TilePrototype {
                sprite: Some(tag), ..
            } = prototype
            {
                match Self::get_sprite(api, tag) {
                    Ok(image) => {
                        atlas_builder.push(tag.clone(), image);
                    }
                    Err(report) => {
                        warn!("Could not load sprite {}. {}", tag, report);
                    }
                }
            }
        }

        WorldRenderer {
            atlas: atlas_builder.export(3),
            color: DrawPipeline::new(
                include_str!("./gl/color.frag.glsl"),
                include_str!("./gl/color.vert.glsl"),
                vec![
                    AttributeDescriptor::new(0, AttributeType::Int(2)),
                    AttributeDescriptor::new(1, AttributeType::Float(3)),
                ],
            ),
            texture: DrawPipeline::new(
                include_str!("./gl/texture.frag.glsl"),
                include_str!("./gl/texture.vert.glsl"),
                vec![
                    AttributeDescriptor::new(0, AttributeType::Int(2)),
                    AttributeDescriptor::new(1, AttributeType::Float(2)),
                ],
            ),
            chunks: Default::default(),
            world_dirty: false
        }
    }

    fn get_sprite(api: &Api, tag: &Tag) -> Result<DynamicImage> {
        let plugin = api.get_plugin(tag.plugin_id()).ok_or_else(|| eyre!(
            "Plugin {} does not exist or is not loaded.",
            tag.plugin_id()
        ))?;
        let data = plugin
            .archive
            .get_asset(&ArchivePath::Asset(tag.name().to_string()))
            .wrap_err(format!("Sprite does not exist {}", tag))?;
        Ok(image::load_from_memory_with_format(data, ImageFormat::Png)?)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.color.resize(width, height);
        self.texture.resize(width, height);
    }

    pub fn submit_chunk(&mut self, api: &Api, pos: ChunkPos, chunk: &Chunk) {
        let mut baked_chunk = BakedChunk::new(api, chunk, &self.atlas);
        self.compile_borders(pos, &mut baked_chunk);
        self.chunks.insert(pos, baked_chunk);
        self.world_dirty = true;
    }

    fn compile_borders(
        &mut self,
        pos: ChunkPos,
        chunk: &mut BakedChunk,
    ) {
        for offset in Direction::all() {
            if let Some(neighbor_pos) = pos.offset(offset) {
                if let Some(neighbor) = self.chunks.get_mut(&neighbor_pos) {
                    let y_offset = offset.offset_y().max(0) as usize * (CHUNK_SIZE - 1);
                    let x_offset = offset.offset_x().max(0) as usize * (CHUNK_SIZE - 1);
                    let y_length = (CHUNK_SIZE - 1) * (offset.offset_x().abs() as usize);
                    let x_length = (CHUNK_SIZE - 1) * (offset.offset_y().abs() as usize);
                    for y in y_offset..=y_length + y_offset {
                        let row = &chunk.tiles.grid[y];
                        // clippy having a stroke
                        #[allow(clippy::needless_range_loop)]
                        for x in x_offset..=x_length + x_offset {
                            let neighbor_sub_pos = ChunkSubPos {
                                x: x as u8,
                                y: y as u8,
                            }.overflowing_offset(offset);

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

                            chunk.tile_neighbors.grid[y][x].set(offset, ty);
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
    pub fn draw(&mut self, prof: &mut Profiler, view: &Player) {
        if self.world_dirty {
            info!("Building mesh");
            // make this off-thread
            let mut builder = VertexBuilder::new();
            for (pos, chunk) in &self.chunks {
                chunk.push(&mut builder, *pos);
            }
            self.texture.submit(builder, RenderLayerStability::Stable).unwrap();
            self.world_dirty = false;
        }

        self.color.draw(prof, view);
        self.texture.draw(prof, view);
    }
}
