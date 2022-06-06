use rsa_core::api::Api;
use rsa_core::math::{AtlasSpace, rect, Rect};
use rsa_core::settings::CHUNK_SIZE;
use rsa_core::ty::{ChunkPos, Direction, Offset};
use rustaria::api::ty::ConnectionType;
use rustaria::chunk::{Chunk, ChunkSystem};
use rustaria::chunk::layer::ChunkLayer;
use rustaria::chunk::layer::tile::TilePrototype;
use crate::Drawer;

use crate::mesh_builder::MeshBuilder;
use crate::neighbor::{NeighborMatrixBuilder, SpriteConnectionKind};
use crate::render::variation;
use crate::ty::PosTex;

pub(crate) struct ChunkTileRenderer {
	renderers: Vec<Option<TileRenderer>>,
}

impl ChunkTileRenderer {
	pub fn new() -> ChunkTileRenderer {
		ChunkTileRenderer {
			renderers: Vec::new()
		}
	}

	pub fn append_mesh(
		&self,
		builder: &mut MeshBuilder<PosTex>,
		storage: &ChunkSystem,
		chunk: &Chunk,
		chunk_pos: ChunkPos,
	) {
		// Compile neighbor matrix
		let mut matrix = NeighborMatrixBuilder::new(self.create_connection_layer(chunk));
		matrix.compile_internal();
		for dir in Direction::values() {
			if let Some(neighbor_pos) = chunk_pos.checked_offset(dir) {
				if let Some(neighbor) = storage.get_chunk(neighbor_pos) {
					let neighbor_matrix = self.create_connection_layer(neighbor);
					matrix.compile_edge(dir, &neighbor_matrix);
				}
			}
		}
		let layer = matrix.export();
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				if let Some(render) = &self.renderers[chunk.tiles.grid[y][x].id.index()] {
					render.append_mesh(
						(chunk_pos.x * CHUNK_SIZE as u32) + x as u32,
						(chunk_pos.y * CHUNK_SIZE as u32) + y as u32,
						builder,
						layer.grid[y][x],
					);
				}
			}
		}
	}

	pub fn reload(&mut self, api: &Api, drawer: &Drawer) {
		let carrier = api.get_carrier();
		let registry = carrier.get::<TilePrototype>();

		self.renderers.clear();
		for prototype in registry.iter() {
			self.renderers.push(TileRenderer::try_new(drawer, prototype));
		}
	}

	fn create_connection_layer(&self, chunk: &Chunk) -> ChunkLayer<ConnectionType> {
		chunk.tiles.map(|tile| {
			self.renderers[tile.id.index()]
				.as_ref()
				.map_or(ConnectionType::Isolated, |tile| tile.connection)
		})
	}
}

pub(crate) struct TileRenderer {
	connection: ConnectionType,
	image: Rect<f32, AtlasSpace>,
	variations: u32,
}

impl TileRenderer {
	pub fn try_new(drawer: &Drawer, prototype: &TilePrototype) -> Option<TileRenderer> {
		let sprite = prototype.sprite.as_ref()?;
		let image = drawer.atlas.get(sprite);
		Some(TileRenderer {
			connection: prototype.connection,
			variations: (image.width() / image.height()).ceil() as u32,
			image,
		})
	}

	pub fn append_mesh(&self, x: u32, y: u32, builder: &mut MeshBuilder<PosTex>, kind: SpriteConnectionKind) {
		let kind = Self::get_pos(kind);
		let tile_height = self.image.height() / 4.0;
		let variation_width = self.image.width() / self.variations as f32;
		let tile_width = self.image.width() / (4.0 * self.variations as f32);
		let variation = variation(x, y) % self.variations;
		builder.push_quad((
			rect::<_, ()>(x as f32, y as f32, 1.0, 1.0),
			rect(
				(self.image.origin.x + (kind[0] as f32 * tile_width))
					+ (variation as f32 * variation_width),
				self.image.origin.y + (kind[1] as f32 * tile_height),
				tile_width,
				tile_height,
			),
		))
	}

	fn get_pos(kind: SpriteConnectionKind) -> [u32; 2] {
		match kind {
			SpriteConnectionKind::Solid => [1, 1],
			SpriteConnectionKind::Lonely => [3, 3],
			SpriteConnectionKind::Vertical => [3, 1],
			SpriteConnectionKind::Horizontal => [1, 3],
			SpriteConnectionKind::CapTop => [3, 0],
			SpriteConnectionKind::CapLeft => [0, 3],
			SpriteConnectionKind::CapDown => [3, 2],
			SpriteConnectionKind::CapRight => [2, 3],
			SpriteConnectionKind::WallTop => [1, 0],
			SpriteConnectionKind::WallLeft => [0, 1],
			SpriteConnectionKind::WallDown => [1, 2],
			SpriteConnectionKind::WallRight => [2, 1],
			SpriteConnectionKind::CornerTopLeft => [0, 0],
			SpriteConnectionKind::CornerTopRight => [2, 0],
			SpriteConnectionKind::CornerDownLeft => [0, 2],
			SpriteConnectionKind::CornerDownRight => [2, 2],
		}
	}
}