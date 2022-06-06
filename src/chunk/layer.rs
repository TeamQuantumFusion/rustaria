use rsa_core::settings::CHUNK_SIZE;
use std::ops::{Index, IndexMut};
use rsa_core::ty::ChunkSubPos;

pub mod tile;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChunkLayer<T> {
	pub grid: [[T; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<T> ChunkLayer<T> {
	pub fn new(values: [[T; CHUNK_SIZE]; CHUNK_SIZE]) -> ChunkLayer<T> {
		ChunkLayer { grid: values }
	}

	pub fn map<V: Default + Copy>(&self, mut func: impl FnMut(&T) -> V) -> ChunkLayer<V> {
		let mut out = ChunkLayer::new_default();
		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				out.grid[y][x] = func(&self.grid[y][x]);
			}
		}
		out
	}
}

impl<T: Copy> ChunkLayer<T> {
	pub fn new_copy(value: T) -> ChunkLayer<T>  {
		Self::new([[value; CHUNK_SIZE]; CHUNK_SIZE])
	}
}

impl<T: Copy + Default> ChunkLayer<T> {
	pub fn new_default() -> ChunkLayer<T>  {
		Self::new_copy(T::default())
	}
}

impl<T> Index<ChunkSubPos> for ChunkLayer<T> {
	type Output = T;

	fn index(&self, index: ChunkSubPos) -> &Self::Output {
		&self.grid[index.y() as usize][index.x() as usize]
	}
}

impl<T> IndexMut<ChunkSubPos> for ChunkLayer<T> {
	fn index_mut(&mut self, index: ChunkSubPos) -> &mut Self::Output {
		&mut self.grid[index.y() as usize][index.x() as usize]
	}
}
