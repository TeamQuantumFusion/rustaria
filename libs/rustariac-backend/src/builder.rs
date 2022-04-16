pub struct VertexBuilder<V: Copy> {
	pub vertex_data: Vec<V>,
	pub index_data: Vec<u32>,
}

impl<V: Copy> VertexBuilder<V> {
	pub fn point(&mut self, value: V) {
		self.index_data.push(self.vertex_data.len() as u32);
		self.vertex_data.push(value);
	}

	pub fn quad(&mut self, value: impl Quadable<V>) {
		let len = self.vertex_data.len() as u32;
		self.index_data
			.extend_from_slice(&[len, 1 + len, 3 + len, 1 + len, 2 + len, 3 + len]);
		self.vertex_data.extend_from_slice(&value.expand());
	}
}
impl<V: Copy> Default for VertexBuilder<V> {
	fn default() -> Self {
		Self {
			vertex_data: Default::default(),
			index_data: Default::default(),
		}
	}
}

pub trait Quadable<V: Copy + Clone> {
	fn expand(self) -> [V; 4];
}

impl<V: Copy + Clone> Quadable<V> for [V; 4] {
	fn expand(self) -> [V; 4] {
		self
	}
}
