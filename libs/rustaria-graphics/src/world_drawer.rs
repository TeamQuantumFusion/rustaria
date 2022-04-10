use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use aloy::vertex::VertexBuilder;
use rustaria::api::Api;
use rustaria::world::chunk::Chunk;
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos};

use crate::Pos;
use crate::renderer::{RenderingHandler, RenderingInstance, RenderLayerConsumer};
use crate::renderer::layer::RenderLayer;
use crate::ty::{Rectangle, Texture, Viewport};
use crate::world_drawer::chunk::BakedChunk;

mod chunk;
mod tile;

// Draws the world and submits to the layer
pub struct WorldDrawer {
	api: Api,
	instance: Arc<RwLock<RenderingInstance>>,
	layer: RenderLayer<(Pos, Texture)>,
	chunks: HashMap<ChunkPos, BakedChunk>,
}

impl WorldDrawer {
	pub fn new(api: &Api, renderer: &mut RenderingHandler) -> WorldDrawer {
		WorldDrawer {
			api: api.clone(),
			layer: renderer.create_layer(),
			chunks: Default::default(),
			instance: renderer.instance()
		}
	}

	pub fn submit(&mut self, pos: ChunkPos, chunk: &Chunk) {
		let instance = self.instance.read().unwrap();
		// todo async mesher
		let mut baked_chunk = BakedChunk::new(&self.api, chunk, &instance.atlas);
		baked_chunk.compile_internal();
		baked_chunk.compile_chunk_borders(&mut self.chunks, pos);
		self.chunks.insert(pos, baked_chunk);
		self.layer.mark_dirty();
	}

	pub fn draw(&mut self, view: &Viewport) {
		if self.layer.dirty() {
			let viewport = view.viewport(self.instance.read().unwrap().screen_y_ratio);

			let mut builder = VertexBuilder::new();
			for (pos, chunk) in &self.chunks {
				let chunk_rect = Rectangle {
					x: pos.x as f32 * CHUNK_SIZE as f32,
					y: pos.y as f32 * CHUNK_SIZE as f32,
					w: CHUNK_SIZE as f32,
					h: CHUNK_SIZE as f32,
				};

				if viewport.overlaps(&chunk_rect) {
					chunk.push(&mut builder, *pos);
				}
			}

			self.layer.supply(builder);
		}
	}
}