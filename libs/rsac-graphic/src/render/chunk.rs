mod tile;

use crate::mesh_builder::MeshBuilder;
use crate::draw::buffer::DrawBuffer;
use crate::ty::PosTex;
use rsa_core::ty::{ChunkPos, Direction, Offset};
use rsa_core::error::Result;
use std::collections::HashMap;
use glium::{Surface, uniform};
use rayon::prelude::*;
use rsa_core::api::Api;
use rustaria::chunk::{ChunkSystem};
use crate::Draw;
use crate::draw::Drawer;
use crate::render::chunk::tile::ChunkTileRenderer;

pub(crate) struct ChunkRenderer {
	buffer: DrawBuffer<PosTex>,
	chunk_meshes: HashMap<ChunkPos, ChunkMesh>,

	// Here for caching purposes
	builder: MeshBuilder<PosTex>,
	dirty_mesh: bool,

	tile_renderer: ChunkTileRenderer
}

impl ChunkRenderer {
	pub fn new(drawer: &Drawer) -> Result<ChunkRenderer> {
		Ok(ChunkRenderer {
			buffer: DrawBuffer::new(drawer)?,
			chunk_meshes: Default::default(),
			builder: MeshBuilder::new(),
			dirty_mesh: false,
			tile_renderer: ChunkTileRenderer::new()
		})
	}

	pub fn dirty_chunk(&mut self, pos: ChunkPos) {
		self.chunk_meshes
			.entry(pos)
			.or_insert_with(|| ChunkMesh {
				builder: MeshBuilder::new(),
				dirty: false
			})
			.dirty = true;
		// Recompile neighbors if they exist
		for dir in Direction::values() {
			if let Some(pos) = pos.checked_offset(dir) {
				if let Some(neighbor) = self.chunk_meshes.get_mut(&pos) {
					neighbor.dirty = true;
				}
			}
		}

		self.dirty_mesh = true;
	}

	pub fn draw(&mut self, draw: &mut Draw, chunks: &ChunkSystem) -> Result<()> {
		let drawer = &draw.system.drawer;
		if self.dirty_mesh {
			// Compile individual meshes
			self.chunk_meshes.par_iter_mut().for_each(|(pos, mesh)| {
				if mesh.dirty {
					mesh.builder.clear();
					if let Some(chunk) = chunks.get_chunk(*pos) {
						self.tile_renderer.append_mesh(&mut mesh.builder, chunks, chunk, *pos);
					}
				}
			});

			self.builder.clear();
			for (_, mesh) in &self.chunk_meshes {
				self.builder.extend(&mesh.builder);
			}
			// TODO error handling here
			self.buffer.submit(drawer, &self.builder).unwrap();
			self.builder.clear();

			// Create mesh
			self.dirty_mesh = false;
		}

		let program = drawer.get_program("pos_tex")?;
		let storage = uniform! {
			screen_y_ratio: drawer.screen_ratio,
			scale: drawer.camera.scale,
			player_pos: drawer.camera.pos.to_array(),
			tex: drawer.atlas.sampler()
		};

		draw.frame.draw(&self.buffer.vertex_buffer, &self.buffer.index_buffer, program, &storage, &glium::draw_parameters::DrawParameters {
			blend: glium::draw_parameters::Blend::alpha_blending(),
			..glium::draw_parameters::DrawParameters::default()
		})?;

		Ok(())
	}

	pub fn reload(&mut self, api: &Api,  drawer: &Drawer) {
		self.chunk_meshes.clear();
		self.builder.clear();
		self.dirty_mesh = true;

		self.tile_renderer.reload(api, drawer);
	}
}

pub struct ChunkMesh {
	builder: MeshBuilder<PosTex>,
	dirty: bool,
}