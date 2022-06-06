/// A MeshBuilder constructs a mesh through a series of points.
/// The vertex data holds the point information and the index data holds the order of the points which may repeat.
#[derive(Clone)]
pub(crate) struct MeshBuilder<V: Clone> {
	pub(crate) vertex_data: Vec<V>,
	pub(crate) index_data: Vec<u32>,
}

impl<V: Clone> MeshBuilder<V> {
	pub fn new() -> MeshBuilder<V> {
		MeshBuilder {
			vertex_data: Vec::new(),
			index_data: Vec::new()
		}
	}


	/// Push a single point to the mesh.
	pub fn push(&mut self, value: V) {
		self.index_data.push(self.vertex_data.len() as u32);
		self.vertex_data.push(value);
	}

	/// Push a triangle to the mesh.
	///
	/// ### Order
	/// - Index: `0`, `1`, `2`
	pub fn push_triangle(&mut self, triangle: impl Triangle<V>) {
		let len = self.vertex_data.len() as u32;
		self.index_data.extend_from_slice(&[len, len + 1, len + 2]);
		self.vertex_data.extend_from_slice(&triangle.expand());
	}

	/// Push a quad to the mesh.
	///
	/// ### Order
	/// - Index: `0`, `1`, `3`, `1`, `2`, `3`
	/// - Vertex: `bottom-left`, `top-left`, `top-right`, `bottom-right`
	pub fn push_quad(&mut self, quad: impl Quad<V>) {
		let len = self.vertex_data.len() as u32;
		self.index_data.extend_from_slice(&[len, len + 1, len + 3, len + 1, len + 2, len + 3]);
		self.vertex_data.extend_from_slice(&quad.expand());
	}


	/// Combines another mesh builder, this also clears the other mesh builder..
	pub fn append(&mut self, builder: &mut MeshBuilder<V>) {
		let len = self.vertex_data.len() as u32;
		for idx in builder.index_data.drain(..) {
			self.index_data.push(len + idx);
		}
		self.vertex_data.append(&mut builder.vertex_data);
	}

	/// Extends another MeshBuilder, this does not mutate the other builder.
	pub fn extend(&mut self, builder: &MeshBuilder<V>) {
		let len = self.vertex_data.len() as u32;
		for idx in &builder.index_data {
			self.index_data.push(len + idx);
		}
		self.vertex_data.extend_from_slice(&builder.vertex_data);
	}

	/// Clears the entire buffer of entries. Useful for reusing a mesh builder and reducing allocations.
	pub fn clear(&mut self) {
		self.vertex_data.clear();
		self.index_data.clear();
	}
}

pub(crate) trait Triangle<V> {
	fn expand(self) -> [V; 3];
}

impl<V> Triangle<V> for [V; 3] {
	fn expand(self) -> [V; 3] {
		self
	}
}

//1, 3 -> 1 ---- 2 <- 4
//        | ordr |
//        |      |
//   0 -> 0 ---- 3 <- 2, 5
// ^ index         index ^
pub(crate) trait Quad<V> {
	fn expand(self) -> [V; 4];
}

impl<V> Quad<V> for [V; 4] {
	fn expand(self) -> [V; 4] {
		self
	}
}
