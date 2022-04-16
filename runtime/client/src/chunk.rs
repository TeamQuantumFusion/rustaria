use std::collections::hash_map::Entry;
use std::collections::HashMap;

use eyre::Result;

use crate::NetworkHandler;
use rustaria::chunk::Chunk;
use rustaria::network::packet::chunk::ClientChunkPacket;
use rustaria::network::packet::chunk::ServerChunkPacket;
use rustaria::network::packet::ClientPacket;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_rendering::chunk_drawer::ChunkDrawer;
use rustaria_util::ty::pos::Pos;
use rustaria_util::ty::CHUNK_SIZE;
use rustaria_util::{info, ty::ChunkPos, warn};
use rustariac_backend::{ty::Viewport, ClientBackend};

pub(crate) struct ChunkHandler {
    backend: ClientBackend,

    chunks: HashMap<ChunkPos, ChunkHolder>,
    chunk_drawer: ChunkDrawer,

    old_chunk: ChunkPos,
    old_zoom: f32,
}

impl ChunkHandler {
    pub fn new(backend: &ClientBackend) -> ChunkHandler {
        ChunkHandler {
            backend: backend.clone(),
            chunks: HashMap::new(),
            chunk_drawer: ChunkDrawer::new(backend),
            old_chunk: ChunkPos::new(60, 420).unwrap(),
            old_zoom: 0.0,
        }
    }

    pub fn packet(&mut self, packet: ServerChunkPacket) -> Result<()> {
        match packet {
            ServerChunkPacket::Provide(chunks) => match chunks.export() {
                Ok(chunks) => {
                    for (pos, chunk) in chunks.chunks {
                        self.chunk_drawer.submit(pos, &chunk)?;
                        self.chunks.insert(pos, ChunkHolder::Active(chunk));
                    }
                }
                Err(chunks) => {
                    warn!("Could not deserialize chunk packet. {chunks}")
                }
            },
        }

        Ok(())
    }

    pub fn tick(&mut self, view: &Viewport, networking: &mut NetworkHandler) -> Result<()> {
        if let Ok(chunk) = ChunkPos::try_from(Pos {
            x: view.position[0],
            y: view.position[1],
        }) {
            if chunk != self.old_chunk || view.zoom != self.old_zoom {
                info!("{:?}", view);
                let width = (view.zoom / CHUNK_SIZE as f32) as i32;
                let height =
                    ((view.zoom * self.backend.screen_y_ratio()) / CHUNK_SIZE as f32) as i32;
                let mut requested = Vec::new();
                for x in -width..width {
                    for y in -height..height {
                        if let Some(pos) = chunk.offset([x, y]) {
                            if let Entry::Vacant(e) = self.chunks.entry(pos) {
                                e.insert(ChunkHolder::Requested);
                                requested.push(pos);
                            }
                        }
                    }
                }

                self.chunk_drawer.mark_dirty();
                if !requested.is_empty() {
                    networking.send(ClientPacket::Chunk(ClientChunkPacket::Request(requested)))?;
                }
                self.old_chunk = chunk;
                self.old_zoom = view.zoom;
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, view: &Viewport) {
        self.chunk_drawer.draw(view);
    }
}

impl Reloadable for ChunkHandler {
    fn reload(&mut self, api: &Api, carrier: &Carrier) {
        self.chunks.clear();
        self.chunk_drawer.reload(api, carrier);
    }
}

pub enum ChunkHolder {
    Active(Chunk),
    Requested,
}
