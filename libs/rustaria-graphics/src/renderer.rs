use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use glfw::Glfw;

use aloy::attribute::{AttributeDescriptor, AttributeType};
use aloy::OpenGlBackend;
use atlas::TextureAtlas;
use rustaria::api::Api;
use rustaria_api::tag::Tag;

use crate::{Pos, RenderPipeline};
use crate::renderer::layer::RenderLayer;
use crate::ty::{Color, Texture, Viewport};

pub mod atlas;
pub mod layer;
pub mod pipeline;

pub struct RenderingHandler {
    instance: Arc<RwLock<RenderingInstance>>,
    pos_color_pipeline: RenderPipeline<(Pos, Color)>,
    pos_texture_pipeline: RenderPipeline<(Pos, Texture)>,
}

pub struct RenderingInstance {
    pub atlas: TextureAtlas,
    pub screen_y_ratio: f32
}

impl RenderingHandler {
    pub fn new(
        api: &Api,
        sprites: HashSet<Tag>,
    ) -> RenderingHandler {
        RenderingHandler {
            instance: Arc::new(RwLock::new(RenderingInstance {
                atlas: TextureAtlas::new(api, sprites),
                screen_y_ratio: 0.0
            })),
            pos_color_pipeline: RenderPipeline::new(
                include_str!("./gl/color.frag.glsl"),
                include_str!("./gl/color.vert.glsl"),
                vec![
                    AttributeDescriptor::new(0, AttributeType::Int(2)),
                    AttributeDescriptor::new(1, AttributeType::Float(3)),
                ],
            ),
            pos_texture_pipeline: RenderPipeline::new(
                include_str!("./gl/texture.frag.glsl"),
                include_str!("./gl/texture.vert.glsl"),
                vec![
                    AttributeDescriptor::new(0, AttributeType::Int(2)),
                    AttributeDescriptor::new(1, AttributeType::Float(2)),
                ],
            ),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pos_texture_pipeline.resize(width, height);
        self.pos_color_pipeline.resize(width, height);
        self.instance.write().unwrap().screen_y_ratio = width as f32 / height as f32;
    }

    pub fn mark_dirty(&mut self) {
        self.pos_texture_pipeline.mark_dirty();
        self.pos_color_pipeline.mark_dirty();
    }

    pub fn instance(&self) ->  Arc<RwLock<RenderingInstance>> {
        self.instance.clone()
    }

    pub fn draw(&mut self, view: &Viewport) {
        self.pos_texture_pipeline.draw(view);
        self.pos_color_pipeline.draw(view);
    }
}

macro_rules! impl_consumer {
    ($TY:ty, $NAME:ident) => {
        impl RenderLayerConsumer<$TY> for RenderingHandler {
            fn create_layer(&mut self) -> RenderLayer<$TY> {
                self.$NAME.create_layer()
            }
        }
    };
}

impl_consumer!((Pos, Color), pos_color_pipeline);
impl_consumer!((Pos, Texture), pos_texture_pipeline);

pub trait RenderLayerConsumer<V: Clone> {
    fn create_layer(&mut self) -> RenderLayer<V>;
}
