use std::collections::hash_map::Entry;
use std::collections::HashMap;

use eyre::Result;
use rustaria_util::ty::{ChunkPos, Offset, CHUNK_SIZE, CHUNK_SIZE_F};

use crate::NetworkHandler;
use rustaria::chunk::Chunk;
use rustaria::network::packet::chunk::ClientChunkPacket;
use rustaria::network::packet::chunk::ServerChunkPacket;
use rustaria::network::packet::ClientPacket;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_util::ty::pos::Pos;
use rustaria_util::{info, warn};
use rustariac_backend::ty::Camera;
use rustariac_backend::ClientBackend;
use rustariac_rendering::chunk_drawer::WorldChunkDrawer;

pub(crate) struct ChunkHandler {
	backend: ClientBackend,

	chunks: HashMap<ChunkPos, ChunkHolder>,
	drawer: WorldChunkDrawer,

	old_chunk: ChunkPos,
	old_zoom: f32,
}

impl ChunkHandler {
	pub fn new(backend: &ClientBackend) -> ChunkHandler {
		ChunkHandler {
			backend: backend.clone(),
			chunks: HashMap::new(),
			drawer: WorldChunkDrawer::new(backend),
			old_chunk: ChunkPos { x: 60, y: 420 },
			old_zoom: 0.0,
		}
	}

	pub fn packet(&mut self, packet: ServerChunkPacket) -> Result<()> {
		match packet {
			ServerChunkPacket::Provide(chunks) => match chunks.export() {
				Ok(chunks) => {
					for (pos, chunk) in chunks.chunks {
						self.drawer.submit(pos, &chunk)?;
						self.chunks.insert(pos, Some(Box::new(chunk)));
					}
				}
				Err(chunks) => {
					warn!("Could not deserialize chunk packet. {chunks}")
				}
			},
		}

		Ok(())
	}

	pub fn tick(&mut self, camera: &Camera, networking: &mut NetworkHandler) -> Result<()> {
		if let Ok(chunk) = ChunkPos::try_from(Pos {
			x: camera.position[0],
			y: camera.position[1],
		}) {
			if chunk != self.old_chunk || camera.zoom != self.old_zoom {
				let width = (camera.zoom / CHUNK_SIZE_F) as i32;
				let height = ((camera.zoom * camera.screen_y_ratio) / CHUNK_SIZE_F) as i32;
				let mut requested = Vec::new();
				for x in -width..width {
					for y in -height..height {
						if let Some(pos) = chunk.checked_offset((x, y)) {
							if let Entry::Vacant(e) = self.chunks.entry(pos) {
								e.insert(None);
								requested.push(pos);
							}
						}
					}
				}

				self.drawer.mark_dirty();
				if !requested.is_empty() {
					networking.send(ClientPacket::Chunk(ClientChunkPacket::Request(requested)))?;
				}
				self.old_chunk = chunk;
				self.old_zoom = camera.zoom;
			}
		}

		Ok(())
	}

	pub fn draw(&mut self, camera: &Camera) {
		self.drawer.draw(camera);
	}
}

impl Reloadable for ChunkHandler {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.chunks.clear();
		self.drawer.reload(api, carrier);
	}
}

pub type ChunkHolder = Option<Box<Chunk>>;
