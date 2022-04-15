use std::collections::HashMap;

use rustaria::{api::{prototype::tile::TilePrototype}, world::chunk::Chunk};
use rustaria_api::Carrier;
use rustaria_util::ty::{ChunkPos, CHUNK_SIZE};
use rustariac_backend::{layer::LayerChannel, ty::{PosTexture, Viewport, Rectangle}, ClientBackend, builder::VertexBuilder};

use self::{chunk::BakedChunk, tile::TileDrawer};

pub mod tile;
pub mod chunk;

pub struct ChunkDrawer {
    backend: ClientBackend,
    carrier: Carrier,
    tile_drawers: Vec<Option<TileDrawer>>,
    layer: LayerChannel<PosTexture>,
    chunks: HashMap<ChunkPos, BakedChunk>,
}


impl ChunkDrawer {
    pub fn new(carrier: &Carrier, backend: &ClientBackend) -> ChunkDrawer {
        let instance = carrier.lock();
        let registry = instance.get_registry::<TilePrototype>();
        
        let mut tile_drawers = Vec::new();
        for prototype in registry.iter() {
            tile_drawers.push(TileDrawer::new(prototype, backend));
        }

        ChunkDrawer {
            tile_drawers,
            layer: backend.instance_mut().backend.new_layer_pos_tex(),
            chunks: HashMap::new(),
            carrier: carrier.clone(),
            backend: backend.clone(),
        }
    }

    pub fn submit(&mut self, pos: ChunkPos, chunk: &Chunk) {
		// todo async mesher
		let mut baked_chunk = BakedChunk::new(&self.carrier, chunk, &self.backend);
		baked_chunk.compile_internal();
		baked_chunk.compile_chunk_borders(&mut self.chunks, pos);
		self.chunks.insert(pos, baked_chunk);
		self.mark_dirty();
	}

	pub fn mark_dirty(&mut self) {
		self.layer.mark_dirty();
	}

    pub fn draw(&mut self, view: &Viewport) {
		if self.layer.dirty() {
			let viewport = view.viewport(self.backend.screen_y_ratio());

			let mut builder = VertexBuilder::new();
			for (pos, chunk) in &self.chunks {
				let chunk_rect = Rectangle {
					x: pos.x as f32 * CHUNK_SIZE as f32,
					y: pos.y as f32 * CHUNK_SIZE as f32,
					width: CHUNK_SIZE as f32,
					height: CHUNK_SIZE as f32,
				};

				if viewport.overlaps(&chunk_rect) {
					chunk.push(&mut builder, &self.tile_drawers, pos);
				}
			}

			self.layer.supply(builder);
		}
	}

    
}