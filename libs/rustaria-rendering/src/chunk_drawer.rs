use std::collections::HashMap;

use rustaria::api::prototype::tile::TilePrototype;
use rustaria::SmartError::CarrierUnavailable;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_util::ty::{CHUNK_SIZE, ChunkPos};
use rustariac_backend::{builder::VertexBuilder, ClientBackend, layer::LayerChannel, ty::{PosTexture, Rectangle, Viewport}};
use eyre::{ContextCompat, Result};
use rustaria::chunk::Chunk;

use self::{chunk::BakedChunk, tile::TileDrawer};

pub mod tile;
pub mod chunk;

pub struct ChunkDrawer {
    backend: ClientBackend,
    carrier: Option<Carrier>,
    tile_drawers: Vec<Option<TileDrawer>>,
    layer: LayerChannel<PosTexture>,
    chunks: HashMap<ChunkPos, BakedChunk>,
}


impl ChunkDrawer {
    pub fn new(backend: &ClientBackend) -> ChunkDrawer {
        ChunkDrawer {
            tile_drawers: Vec::new(),
            layer: backend.instance_mut().backend.new_layer_pos_tex(),
            chunks: HashMap::new(),
            carrier: None,
            backend: backend.clone(),
        }
    }

    pub fn submit(&mut self, pos: ChunkPos, chunk: &Chunk) -> Result<()> {
	    let carrier = self.carrier.as_ref().wrap_err(CarrierUnavailable)?;
		// todo async mesher
		let mut baked_chunk = BakedChunk::new(carrier, chunk, &self.backend);
		baked_chunk.compile_internal();
		baked_chunk.compile_chunk_borders(&mut self.chunks, pos);
		self.chunks.insert(pos, baked_chunk);
		self.mark_dirty();
	    Ok(())
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

impl Reloadable for ChunkDrawer  {
	fn reload(&mut self, _: &Api, carrier: &Carrier) {
		self.chunks.clear();
		self.carrier = Some(carrier.clone());
		let instance = carrier.lock();
		let registry = instance.get_registry::<TilePrototype>();

		self.tile_drawers.clear();
		for prototype in registry.iter() {
			self.tile_drawers.push(TileDrawer::new(prototype, &self.backend));
		}

		self.mark_dirty();
	}
}