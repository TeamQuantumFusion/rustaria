use std::sync::{Arc, RwLock, Weak};

use aloy::attribute::AttributeDescriptor;
use aloy::buffer::{Buffer, BufferAccess, BufferType, BufferUsage, DrawMode, VertexPipeline};
use aloy::program::Program;
use aloy::vertex::VertexBuilder;

#[derive(Clone)]
/// A RenderLayer is a cheaply copyable object for rendering vertexes.
/// When the layer is marked dirty the intended drawer for that layer should create the mesh and then supply the layer its contents.
pub struct RenderLayer<V: Clone>(pub(crate) Arc<RwLock<RenderLayerData<V>>>);

impl<V: Clone> RenderLayer<V> {
    /// Check if the layer should be supplied with a mesh.
    #[must_use]
    pub fn dirty(&self) -> bool {
        self.0.read().unwrap().dirty
    }

    /// Force a layer to be re-meshed.
    pub fn mark_dirty(&mut self) {
        self.0.write().unwrap().dirty = true;
    }

    /// Supply the layer with a mesh. Should only be called if the layer is marked dirty.
    pub fn supply(&mut self, builder: VertexBuilder<V>) {
        let mut data = self.0.write().unwrap();
        data.new_data = Some(builder);
        data.dirty = false;
    }
}

pub(crate) struct RenderLayerData<V: Clone> {
    pub(crate) dirty: bool,
    pub(crate) new_data: Option<VertexBuilder<V>>,
}

pub(crate) struct RenderLayerDrawer<V: Clone> {
    pub pipeline: VertexPipeline,
    pub vertices: Buffer<V>,
    pub indices: Buffer<u32>,
    pub reference: Weak<RwLock<RenderLayerData<V>>>
}

impl<V: Clone> RenderLayerDrawer<V> {
    pub fn new(attributes: Vec<AttributeDescriptor>, layer: &RenderLayer<V>) -> RenderLayerDrawer<V> {
        let vertices = Buffer::<V>::create(
            BufferType::Vertex(attributes),
            BufferUsage::Static,
            BufferAccess::Draw,
            0,
        );

        let indices = Buffer::<u32>::create(
            BufferType::index::<u32>(),
            BufferUsage::Static,
            BufferAccess::Draw,
            0,
        );

        let mut pipeline = VertexPipeline::new();
        pipeline.bind_buffer(&vertices);
        pipeline.bind_buffer(&indices);

        RenderLayerDrawer {
            pipeline,
            vertices,
            indices,
            reference: Arc::downgrade(&layer.0)
        }
    }

    pub fn draw(&mut self, program: &Program) -> Option<()> {
       if let Some(data) =  self.reference.upgrade() {
           if let Some(builder) = data.write().unwrap().new_data.take() {
               self.indices.set(&builder.indices);
               self.vertices.set(&builder.vertices);
           }
           program.draw(&self.pipeline, 0..self.indices.get_elements(), DrawMode::Triangle);
           Some(())
       } else  {
           None
       }
    }
}