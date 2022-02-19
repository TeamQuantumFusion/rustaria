use std::rc::Rc;
use std::sync::Arc;
use std::thread::Builder;
use eyre::ContextCompat;
use rsa_api::prototypes::{Prototype, TilePrototype, WallPrototype};
use rsa_api::registry::Tag;
use rsa_api::Rustaria;
use crate::Chunk;
use crate::chunk::ChunkGrid;


pub struct WorldGenerator {
    generator: Arc<ChunkGenerator>,
    thread_pool: Runtime,
    done_send: UnboundedSender<(ChunkPos, Chunk)>,
    done_receiver: UnboundedReceiver<(ChunkPos, Chunk)>,
}

impl WorldGenerator {
    pub fn new(seed: u64, default_tile: &Id, default_wall: &Tag, api: &Rustaria) -> eyre::Result<WorldGenerator> {
        let (done_send, done_receiver) = tokio::sync::mpsc::unbounded_channel();
        Ok(WorldGenerator {
            generator: Arc::new(ChunkGenerator {
                seed,
                default_tile: (*default_tile, api.tiles.get_entry(default_tile).wrap_err("Could not find tile.")?.clone()),
                default_wall: (*default_wall, api.walls.get_entry(default_wall).wrap_err("Could not find wall.")?.clone()),
            }),
            thread_pool: Builder::new_multi_thread().build()?,
            done_send,
            done_receiver,
        })
    }

    pub fn request_chunk(&self, pos: ChunkPos) {
        let sender = self.done_send.clone();
        let generator = self.generator.clone();

        self.thread_pool.spawn(async move {
            sender.send(generator.generate_chunk(pos))
        });
    }

    pub fn poll_chunk(&mut self) -> Option<(ChunkPos, Chunk)> {
        self.done_receiver.try_recv().ok()
    }
}

pub struct ChunkGenerator {
    seed: u64,
    default_tile: (Id, TilePrototype),
    default_wall: (Id, WallPrototype),
}

impl ChunkGenerator {
    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> (ChunkPos, Chunk) {
        (chunk_pos, Chunk {
            tiles: ChunkGrid::new(self.default_tile.1.create(self.default_tile.0)),
            walls: ChunkGrid::new(self.default_wall.1.create(self.default_wall.0)),
        })
    }
}