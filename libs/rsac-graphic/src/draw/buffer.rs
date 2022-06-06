use glium::{IndexBuffer, Vertex, VertexBuffer};
use glium::index::PrimitiveType;
use crate::mesh_builder::MeshBuilder;
use crate::draw::Drawer;

pub(crate) struct DrawBuffer<V: Copy + Clone + Vertex> {
	pub(crate) vertex_buffer: VertexBuffer<V>,
	pub(crate) index_buffer: IndexBuffer<u32>,
}

impl<V: Copy + Clone + Vertex> DrawBuffer<V> {
	pub fn new(drawer: &Drawer) -> Result<DrawBuffer<V>, BufferCreationError> {
		Ok(DrawBuffer {
			vertex_buffer: VertexBuffer::empty_dynamic(&drawer.context, 1024)?,
			index_buffer: IndexBuffer::empty_dynamic(&drawer.context, PrimitiveType::TrianglesList, 1024)?,
		})
	}

	pub fn submit(&mut self, renderer: &Drawer, mesh_builder: &MeshBuilder<V>) -> Result<(), BufferCreationError> {
		if !mesh_builder.vertex_data.is_empty() {
			if let Some(slice) = self
				.vertex_buffer
				.slice_mut(0..mesh_builder.vertex_data.len())
			{
				slice.write(&mesh_builder.vertex_data);
			} else {
				self.vertex_buffer = VertexBuffer::dynamic(&renderer.context, &mesh_builder.vertex_data)?;
			}

			if let Some(slice) = self
				.index_buffer
				.slice_mut(0..mesh_builder.index_data.len())
			{
				slice.write(&mesh_builder.index_data);
			} else {
				self.index_buffer = IndexBuffer::dynamic(
					&renderer.context,
					PrimitiveType::TrianglesList,
					&mesh_builder.index_data,
				)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, thiserror::Error)]
pub enum BufferCreationError {
	#[error(transparent)]
	Index(#[from] glium::index::BufferCreationError),
	#[error(transparent)]
	Vertex(#[from] glium::vertex::BufferCreationError),
}
