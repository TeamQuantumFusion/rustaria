use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use aloy::vertex::VertexBuilder;
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::api::rendering::RenderingSystem;
use rustaria::api::Api;
use rustaria::world::chunk::Chunk;
use rustaria::world::entity::query::IntoQuery;
use rustaria::world::entity::world::Entity;
use rustaria::world::entity::{EntityHandler, IdComp, PositionComp, Read};
use rustaria_api::RawId;
use rustaria_util::info;
use rustaria_util::ty::{ChunkPos, CHUNK_SIZE};

use crate::renderer::layer::RenderLayer;
use crate::renderer::{RenderLayerConsumer, RenderingHandler, RenderingInstance};
use crate::ty::{Rectangle, Texture, Viewport};
use crate::world_drawer::chunk::BakedChunk;
use crate::world_drawer::entity::EntityRenderingSystemDrawer;
use crate::Pos;

mod chunk;
mod entity;
mod tile;

// Draws the world and submits to the layer
pub struct WorldDrawer {
    api: Api,
    instance: Arc<RwLock<RenderingInstance>>,
    chunk_layer: RenderLayer<(Pos, Texture)>,
    entity_layer: RenderLayer<(Pos, Texture)>,
    chunks: HashMap<ChunkPos, BakedChunk>,
    entities: HashMap<RawId, EntityRenderingSystemDrawer>,
}

impl WorldDrawer {
    pub fn new(api: &Api, renderer: &mut RenderingHandler) -> WorldDrawer {
        let guard = api.instance();
        let mut entities = HashMap::new();
	    let instance = renderer.instance();
	    {
		    let atlas = &instance.read().unwrap().atlas;
		    for (id, entries) in guard.get_registry::<EntityPrototype>().id_entries() {
			    for rendering_system in &entries.rendering {
				    if let Some(rendering_system) = EntityRenderingSystemDrawer::new(rendering_system, atlas) {
					    entities.insert(
						    id as RawId,
						    rendering_system,
					    );
				    }
			    }
		    }
	    }

	    WorldDrawer {
            api: api.clone(),
		    entity_layer: renderer.create_layer(),
		    chunk_layer: renderer.create_layer(),
            chunks: Default::default(),
            instance,
            entities,
        }
    }

    pub fn submit_chunk(&mut self, pos: ChunkPos, chunk: &Chunk) {
        let instance = self.instance.read().unwrap();
        // todo async mesher
        let mut baked_chunk = BakedChunk::new(&self.api, chunk, &instance.atlas);
        baked_chunk.compile_internal();
        baked_chunk.compile_chunk_borders(&mut self.chunks, pos);
        self.chunks.insert(pos, baked_chunk);
        self.chunk_layer.mark_dirty();
    }

    pub fn draw(&mut self, view: &Viewport, entities: &EntityHandler) {
        if self.chunk_layer.dirty() {
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

            self.chunk_layer.supply(builder);
        }

        // entity rendering
        let mut builder = VertexBuilder::new();
        let mut query = <(&PositionComp, &IdComp)>::query();
	    for (pos, id) in query.iter(&entities.universe) {
		    if let Some(renderer) = self.entities.get(&id.0) {
			    renderer.push(&mut builder, (pos.position.x, pos.position.y));
		    }
	    }
        self.entity_layer.supply(builder);
    }
}
