use std::collections::HashSet;
use std::sync::Arc;

use crossbeam::channel::{unbounded, Receiver, Sender};
use eyre::ContextCompat;
use rayon::ThreadPool;

use rustaria_api::ty::{Prototype, Tag};
use rustaria_api::{Carrier, Reloadable};
use rustaria_util::error;
use rustaria_util::ty::{ChunkPos, ChunkSubPos, CHUNK_SIZE};

use crate::api::prototype::tile::TilePrototype;
use crate::chunk::{Chunk, ChunkLayer};
use crate::manager::world_gen;
use crate::SmartError;

pub struct WorldGenManager {
    carrier: Option<Carrier>,
    thread_pool: Arc<ThreadPool>,
    submitted_chunks: HashSet<ChunkPos>,
    channel: (Sender<(Chunk, ChunkPos)>, Receiver<(Chunk, ChunkPos)>),
}

impl WorldGenManager {
    pub fn new(thread_pool: Arc<ThreadPool>) -> eyre::Result<WorldGenManager> {
        Ok(WorldGenManager {
            carrier: None,
            thread_pool,
            submitted_chunks: Default::default(),
            channel: unbounded(),
        })
    }

    pub fn request_chunk(&mut self, pos: ChunkPos) -> eyre::Result<()> {
        if !self.submitted_chunks.contains(&pos) {
            self.submitted_chunks.insert(pos);
            let carrier = self
                .carrier
                .clone()
                .wrap_err(SmartError::CarrierUnavailable)?;
            let sender = self.channel.0.clone();
            self.thread_pool.spawn(move || {
                let api = carrier;
                match world_gen::generate_chunk(&api, pos) {
                    Ok(chunk) => sender.send((chunk, pos)).unwrap(),
                    Err(err) => {
                        error!("Could not generate chunk {}", err);
                    }
                };
            });
        }

        Ok(())
    }

    pub fn poll_chunks<C: FnMut(Chunk, ChunkPos)>(&mut self, mut func: C) {
        while let Ok((chunk, pos)) = self.channel.1.try_recv() {
            self.submitted_chunks.remove(&pos);
            func(chunk, pos);
        }
    }
}

// we should prob convert chunks incase a new entry now exists.
// that needs world saving logic however sooooo
impl Reloadable for WorldGenManager {
    fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
        self.carrier = Some(carrier.clone());
    }
}

fn generate_chunk(stack: &Carrier, pos: ChunkPos) -> eyre::Result<Chunk> {
    let instance = stack.lock();
    let tiles = instance.get_registry::<TilePrototype>();

    // We do a touch of unwrapping.
    let id = tiles
        .get_id(&Tag::new("rustaria:air".to_string())?)
        .wrap_err("lol")?;
    let air = tiles.get_prototype(id).unwrap().create(id);

    let id = tiles
        .get_id(&Tag::new("rustaria:dirt".to_string())?)
        .wrap_err("lol")?;
    let dirt = tiles.get_prototype(id).wrap_err("lmao")?.create(id);

    let mut chunk = Chunk {
        tiles: ChunkLayer::new([[air; CHUNK_SIZE]; CHUNK_SIZE]),
    };

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            if ((y + (pos.y as usize * CHUNK_SIZE)) ^ (x + (pos.x as usize * CHUNK_SIZE))) % 5 == 0
            {
                chunk.tiles.put(dirt, ChunkSubPos::new(x as u8, y as u8));
            }
        }
    }

    Ok(chunk)
}
