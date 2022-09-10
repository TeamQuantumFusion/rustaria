use std::ops::Add;

use rsa_core::math::Rect;

/// A MeshBuilder constructs a mesh through a series of points.
/// The vertex data holds the point information and the index data holds the order of the points which may repeat.
#[derive(Clone)]
pub struct MeshBuilder<V: Clone> {
	pub(crate) vertex: Vec<V>,
	pub(crate) index: Vec<u32>,
}

impl<V: Clone> MeshBuilder<V> {
	pub fn new() -> MeshBuilder<V> {
		MeshBuilder {
			vertex: Vec::new(),
			index: Vec::new(),
		}
	}

	/// Push a single point to the mesh.
	pub fn push(&mut self, value: V) {
		self.index.push(self.vertex.len() as u32);
		self.vertex.push(value);
	}

	/// Push a triangle to the mesh.
	///
	/// ### Order
	/// - Index: `0`, `1`, `2`
	pub fn push_triangle(&mut self, triangle: impl Triangle<V>) {
		let len = self.vertex.len() as u32;
		self.index.extend_from_slice(&[len, len + 1, len + 2]);
		self.vertex.extend_from_slice(&triangle.expand());
	}

	/// Push a quad to the mesh.
	///
	/// ### Order
	/// - Index: `0`, `1`, `3`, `1`, `2`, `3`
	/// - Vertex: `bottom-left`, `top-left`, `top-right`, `bottom-right`
	pub fn push_quad(&mut self, quad: impl Quad<V>) {
		let len = self.vertex.len() as u32;
		self.index
			.extend_from_slice(&[len, len + 1, len + 3, len + 1, len + 2, len + 3]);
		self.vertex.extend_from_slice(&quad.expand());
	}

	/// Combines another mesh builder, this also clears the other mesh builder..
	pub fn append(&mut self, builder: &mut MeshBuilder<V>) {
		let len = self.vertex.len() as u32;

		self.index.reserve(builder.index.len());
		for idx in builder.index.drain(..) {
			self.index.push(len + idx);
		}
		self.vertex.append(&mut builder.vertex);
	}

	/// Extends another MeshBuilder, this does not mutate the other builder.
	pub fn extend(&mut self, builder: &MeshBuilder<V>) {
		let len = self.vertex.len() as u32;
		self.index.reserve(builder.index.len());
		for idx in &builder.index {
			self.index.push(len + idx);
		}

		self.vertex.extend_from_slice(&builder.vertex);
	}

	/// Clears the entire buffer of entries. Useful for reusing a mesh builder and reducing allocations.
	pub fn clear(&mut self) {
		self.vertex.clear();
		self.index.clear();
	}
}

pub trait Triangle<V> {
	fn expand(self) -> [V; 3];
}

impl<V> Triangle<V> for [V; 3] {
	fn expand(self) -> [V; 3] { self }
}

//1, 3 -> 1 ---- 2 <- 4
//        | ordr |
//        |      |
//   0 -> 0 ---- 3 <- 2, 5
// ^ index         index ^
pub trait Quad<V> {
	fn expand(self) -> [V; 4];
}

impl<V> Quad<V> for [V; 4] {
	fn expand(self) -> [V; 4] { self }
}

impl<U, T: Copy + Add<T, Output = T>> Quad<[T; 2]> for Rect<T, U> {
	fn expand(self) -> [[T; 2]; 4] {
		[
			[self.min_x(), self.min_y()],
			[self.min_x(), self.max_y()],
			[self.max_x(), self.max_y()],
			[self.max_x(), self.min_y()],
		]
	}
}

impl<V: Clone> Quad<V> for V {
	fn expand(self) -> [V; 4] { [self.clone(), self.clone(), self.clone(), self] }
}
