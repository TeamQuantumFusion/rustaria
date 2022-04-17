use std::sync::{Arc, RwLock};

use crate::builder::VertexBuilder;

pub struct LayerChannel<V: Copy + Clone>(pub Arc<RwLock<LayerChannelData<V>>>);

impl<V: Clone + Copy> LayerChannel<V> {
	/// Check if the layer should be supplied with a mesh.
	#[must_use = "if you are trying to make it dirty use mark_dirty() instead."]
	pub fn dirty(&self) -> bool {
		self.0.read().unwrap().dirty
	}

	/// Force a layer to be re-meshed.
	pub fn mark_dirty(&mut self) {
		self.0.write().unwrap().dirty = true;
	}

	/// Supply the layer with a mesh.
	pub fn supply(&mut self, builder: VertexBuilder<V>) {
		let mut data = self.0.write().unwrap();
		data.new_data = Some(builder);
		data.dirty = false;
	}
}

pub struct LayerChannelData<V: Copy + Clone> {
	pub dirty: bool,
	pub new_data: Option<VertexBuilder<V>>,
}
