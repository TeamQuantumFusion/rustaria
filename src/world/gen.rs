use std::collections::HashSet;
use std::sync::Arc;

use crossbeam::channel::{unbounded, Receiver, Sender};
use eyre::{ContextCompat, Result};
use rayon::ThreadPool;

use rustaria_api::{Carrier, Reloadable};
use rustaria_util::{error, debug};
use rustaria_util::ty::ChunkPos;

use crate::err::SmartError;
use crate::world::chunk::Chunk;
use crate::world::gen::chunk::generate_chunk;

mod chunk;

pub struct ChunkGenerator {
    carrier: Option<Carrier>,
    thread_pool: Arc<ThreadPool>,
    submitted_chunks: HashSet<ChunkPos>,
    channel: (Sender<(Chunk, ChunkPos)>, Receiver<(Chunk, ChunkPos)>),
}

impl ChunkGenerator {
    pub fn new(thread_pool: Arc<ThreadPool>) -> Result<ChunkGenerator> {
        Ok(ChunkGenerator {
            carrier: None,
            thread_pool,
            submitted_chunks: Default::default(),
            channel: unbounded(),
        })
    }

    pub fn request_chunk(&mut self, pos: ChunkPos) -> Result<()> {
        if !self.submitted_chunks.contains(&pos) {
            self.submitted_chunks.insert(pos);
            let carrier = self
                .carrier
                .clone()
                .wrap_err(SmartError::CarrierUnavailable)?;
            let sender = self.channel.0.clone();
            self.thread_pool.spawn(move || {
                let api = carrier;
                match generate_chunk(&api, pos) {
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
impl Reloadable for ChunkGenerator {
    fn reload(&mut self, _: &rustaria_api::Api, carrier: &Carrier) {
		debug!("Reloaded ChunkGenerator");
        self.carrier = Some(carrier.clone());
    }
}
