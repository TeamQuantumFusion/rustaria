use std::collections::HashSet;
use std::sync::Arc;

use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::ThreadPool;
use rsa_core::api::carrier::Carrier;
use rsa_core::api::{Api, Reloadable};

use rsa_core::ty::{ChunkPos, ChunkSubPos, Prototype, Tag};
use rsa_core::error::ContextCompat;
use rsa_core::error::Result;
use rsa_core::logging::error;
use rsa_core::settings::CHUNK_SIZE;

use crate::api::prototype::tile::TilePrototype;
use crate::chunk::{Chunk, ChunkLayer};
use crate::SmartError;

pub struct WorldGeneration {
	carrier: Option<Carrier>,
	thread_pool: Arc<ThreadPool>,
	submitted_chunks: HashSet<ChunkPos>,

	tx: Sender<(Chunk, ChunkPos)>,
	rx: Receiver<(Chunk, ChunkPos)>,
}

impl WorldGeneration {
	pub fn new(thread_pool: Arc<ThreadPool>) -> Result<WorldGeneration> {
		let (tx, rx) = unbounded();
		Ok(WorldGeneration {
			carrier: None,
			thread_pool,
			submitted_chunks: Default::default(),
			tx,
			rx,
		})
	}

	pub fn request_chunk(&mut self, pos: ChunkPos) -> Result<()> {
		if !self.submitted_chunks.contains(&pos) {
			self.submitted_chunks.insert(pos);
			let carrier = self
				.carrier
				.clone()
				.wrap_err(SmartError::CarrierUnavailable)?;

			let sender = self.tx.clone();
			self.thread_pool.spawn(move || {
				let api = carrier;
				match generate_chunk(&api, pos) {
					Ok(chunk) => sender.send((chunk, pos)).unwrap(),
					Err(err) => {
						error!(target: "misc@rustaria", "Could not generate chunks {err}");
					}
				};
			});
		}

		Ok(())
	}

	pub fn poll_chunks<C: FnMut(Chunk, ChunkPos)>(&mut self, mut func: C) {
		while let Ok((chunk, pos)) = self.rx.try_recv() {
			self.submitted_chunks.remove(&pos);
			func(chunk, pos);
		}
	}
}

// we should prob convert chunks incase a new entry now exists.
// that needs world saving logic however sooooo
impl Reloadable for WorldGeneration {
	fn reload(&mut self, api: &Api) {
		self.carrier = Some(api.get_carrier());
	}
}

fn generate_chunk(stack: &Carrier, pos: ChunkPos) -> Result<Chunk> {
	let tiles = stack.get::<TilePrototype>();

	// We do a touch of unwrapping.
	let id = tiles
		.id_from_tag(&Tag::rsa("air"))
		.wrap_err("lol")?;
	let air = tiles.prototype_from_id(id).create(id);

	let id = tiles
		.id_from_tag(&Tag::rsa("dirt"))
		.wrap_err("lol")?;
	let dirt = tiles.prototype_from_id(id).create(id);

	let mut chunk = Chunk {
		tiles: ChunkLayer::new([[air; CHUNK_SIZE]; CHUNK_SIZE]),
	};

	for y in 0..CHUNK_SIZE {
		for x in 0..CHUNK_SIZE {
			let x_world = (x + (pos.x as usize * CHUNK_SIZE));
			let y_world = (y + (pos.y as usize * CHUNK_SIZE));
			if x_world > 50 || y_world < 4 {
				let pos = ChunkSubPos::new(x as u8, y as u8);
				chunk.tiles[pos] = dirt;
			}
		}
	}

	Ok(chunk)
}
