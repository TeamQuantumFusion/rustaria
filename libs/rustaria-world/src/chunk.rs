use typemap::{Key, TypeMap};
use rustaria_util::ContextCompat;
use rustaria_util::ty::{CHUNK_SIZE, ChunkSubPos};

pub struct Chunk {
	layers: TypeMap
}

impl Chunk {
	pub fn fill<T: 'static + Copy + Clone>(&mut self, layer: ChunkLayer<T>) {
		self.layers.insert::<ChunkLayer<T>>(layer);
	}

	pub fn layer<T: 'static + Copy + Clone>(&self) -> &ChunkLayer<T> {
		self.layers.get::<ChunkLayer<T>>().wrap_err("Could not find layer").unwrap()
	}
}

pub struct ChunkLayer<T: Copy + Clone> {
	values: [T; CHUNK_SIZE]
}

impl<T: Copy + Clone> ChunkLayer<T> {
	pub fn new(default: T) -> ChunkLayer<T>{
		ChunkLayer {
			values: [default; CHUNK_SIZE]
		}
	}

	#[inline(always)]
	pub fn get(&self, pos: ChunkSubPos) -> &T {
		&self.values[pos.index()]
	}

	#[inline(always)]
	pub fn get_mut(&mut self, pos: ChunkSubPos) -> &T {
		&mut self.values[pos.index()]
	}

	#[inline(always)]
	pub fn put(&mut self, value: T, pos: ChunkSubPos) {
		self.values[pos.index()] = value;
	}
}

impl<T: 'static + Copy + Clone> Key for ChunkLayer<T> {
	type Value = ChunkLayer<T>;
}
