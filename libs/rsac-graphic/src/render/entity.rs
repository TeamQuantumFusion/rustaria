mod renderer;

use crate::draw::buffer::DrawBuffer;
use crate::Drawer;
use crate::render::entity::renderer::EntityTypeRenderer;
use crate::ty::PosTex;
use rsa_core::error::Result;

pub(crate) struct EntityRenderer {
	buffer: DrawBuffer<PosTex>,

	// entity id to EntityTypeRenderer
	renderers: Vec<Option<EntityTypeRenderer>>,
}

impl EntityRenderer {
	pub fn new(drawer: &Drawer) -> Result<EntityRenderer> {
		Ok(EntityRenderer {
			buffer: DrawBuffer::new(drawer)?,
			renderers: vec![]
		})
	}
}