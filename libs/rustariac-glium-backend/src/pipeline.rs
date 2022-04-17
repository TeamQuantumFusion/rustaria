use std::sync::{Arc, RwLock, Weak};

use glium::{
	index::PrimitiveType, program::ProgramCreationInput, uniforms::Uniforms, DrawParameters, Frame,
	IndexBuffer, Program, Surface, Vertex, VertexBuffer,
};
use rustariac_backend::layer::{LayerChannel, LayerChannelData};

use crate::engine::GliumBackendEngine;

pub struct LayerPipeline<V: Clone + Copy + Vertex> {
	program: Program,
	layers: Vec<LayerReceiver<V>>,
}

impl<V: Clone + Copy + Vertex> LayerPipeline<V> {
	pub fn new(facade: &GliumBackendEngine, frag: &str, vert: &str) -> LayerPipeline<V> {
		Self {
			program: Program::new(
				facade,
				ProgramCreationInput::SourceCode {
					vertex_shader: vert,
					tessellation_control_shader: None,
					tessellation_evaluation_shader: None,
					geometry_shader: None,
					fragment_shader: frag,
					transform_feedback_varyings: None,
					outputs_srgb: false,
					uses_point_size: false,
				},
			)
			.unwrap(),
			layers: vec![],
		}
	}

	pub fn mark_dirty(&mut self) {
		for layer in &self.layers {
			if let Some(data) = layer.reference.upgrade() {
				data.write().unwrap().dirty = true;
			}
		}
	}

	pub fn create_layer(&mut self, facade: &GliumBackendEngine) -> LayerChannel<V> {
		let layer = LayerChannel(Arc::new(RwLock::new(LayerChannelData {
			dirty: true,
			new_data: None,
		})));

		self.layers.push(LayerReceiver::new(facade, &layer));
		layer
	}

	pub fn draw<U: Uniforms>(
		&mut self,
		facade: &GliumBackendEngine,
		frame: &mut Frame,
		uniforms: &U,
		draw_parameters: &DrawParameters,
	) {
		for drawer in &mut self.layers {
			drawer.draw(facade, frame, &self.program, uniforms, draw_parameters);
		}
	}
}

pub struct LayerReceiver<V: Clone + Copy + Vertex> {
	pub vertex_data: VertexBuffer<V>,
	pub index_data: IndexBuffer<u32>,
	pub reference: Weak<RwLock<LayerChannelData<V>>>,
	pub elements: usize,
}

impl<V: Clone + Copy + Vertex> LayerReceiver<V> {
	pub fn new(facade: &GliumBackendEngine, layer: &LayerChannel<V>) -> LayerReceiver<V> {
		LayerReceiver {
			vertex_data: VertexBuffer::immutable(facade, &[]).unwrap(),
			index_data: IndexBuffer::immutable(facade, PrimitiveType::TrianglesList, &[]).unwrap(),
			reference: Arc::downgrade(&layer.0),
			elements: 0,
		}
	}

	pub fn draw<U: Uniforms>(
		&mut self,
		facade: &GliumBackendEngine,
		frame: &mut Frame,
		program: &Program,
		uniforms: &U,
		draw_parameters: &DrawParameters,
	) -> Option<()> {
		if let Some(data) = self.reference.upgrade() {
			if let Some(builder) = data.write().unwrap().new_data.take() {
				self.vertex_data = VertexBuffer::immutable(facade, &builder.vertex_data).unwrap();
				self.index_data = IndexBuffer::immutable(
					facade,
					PrimitiveType::TrianglesList,
					&builder.index_data,
				)
				.unwrap();
				self.elements = self.index_data.len();
			}
			if self.elements > 0 {
				frame
					.draw(
						&self.vertex_data,
						&self.index_data,
						program,
						uniforms,
						draw_parameters,
					)
					.unwrap();
			}
			Some(())
		} else {
			None
		}
	}
}
