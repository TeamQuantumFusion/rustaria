use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use aloy::attribute::AttributeDescriptor;

use aloy::program::Program;
use aloy::uniform::Uniform;

use crate::renderer::layer::{RenderLayer, RenderLayerData, RenderLayerDrawer};
use crate::ty::Viewport;

/// A RenderingPipeline holds a specific OpenGL shader and manages the layers that are bound to that type.
pub struct RenderPipeline<V: Clone + Debug> {
    program: Program,
    screen_y_ratio: Uniform<f32>,
    zoom: Uniform<f32>,
    player_pos: Uniform<[f32; 2]>,

    attributes: Vec<AttributeDescriptor>,
    layers: Vec<RenderLayerDrawer<V>>,
}

impl<V: Clone + Debug> RenderPipeline<V> {
    /// Create the pipeline with its vertex attributes.
    pub fn new(frag: &str, vert: &str, attributes: Vec<AttributeDescriptor>) -> RenderPipeline<V> {
        let mut program = Program::new(vert.to_string(), frag.to_string());
        let screen_y_ratio = program.get_uniform("screen_y_ratio").unwrap();
        let zoom = program.get_uniform("zoom").unwrap();
        let player_pos = program.get_uniform("player_pos").unwrap();
        Self {
            program,
            screen_y_ratio,
            zoom,
            player_pos,
            attributes,
            layers: vec![],
        }
    }

    // Creates a layer and holds a weak reference to it so it can draw it.
    pub fn create_layer(&mut self) -> RenderLayer<V> {
        let layer = RenderLayer(Arc::new(RwLock::new(RenderLayerData {
            dirty: true,
            new_data: None,
        })));

        self.layers
            .push(RenderLayerDrawer::new(self.attributes.clone(), &layer));
        layer
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_y_ratio.set_value(width as f32 / height as f32);
        // some may cull on layer drawing so we make all of the layers redraw.
        self.mark_dirty();
    }

    pub fn mark_dirty(&mut self) {
        for drawer in &self.layers {
            if let Some(drawer) = drawer.reference.upgrade() {
                drawer.write().unwrap().dirty = true;
            }
        }
    }

    pub fn draw(&mut self, view: &Viewport) {
        self.player_pos.set_value([view.pos.x, view.pos.y]);
        self.zoom.set_value(view.zoom);

        for drawer in &mut self.layers {
            drawer.draw(&self.program);
        }
    }
}
