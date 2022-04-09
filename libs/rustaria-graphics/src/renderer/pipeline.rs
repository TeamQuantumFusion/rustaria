use std::any::type_name;
use std::fmt::Debug;
use aloy::attribute::AttributeDescriptor;
use aloy::buffer::{Buffer, BufferAccess, BufferType, BufferUsage, DrawMode, VertexPipeline};
use aloy::program::Program;
use aloy::uniform::Uniform;
use rustaria_util::{debug, info, Result};

use crate::{Profiler, RenderLayerStability, VertexBuilder};
use crate::ty::Player;

pub struct DrawPipeline<V: Clone + Debug> {
    program: Program,
    dynamic_stack: BufferStack<V>,
    static_stack: BufferStack<V>,
    screen_y_ratio: Uniform<f32>,
    zoom: Uniform<f32>,
    player_pos: Uniform<[f32; 2]>,
}

impl<V: Clone  + Debug> DrawPipeline<V> {
    pub fn new(frag: &str, vert: &str, attributes: Vec<AttributeDescriptor>) -> DrawPipeline<V> {
        let mut program = Program::new(vert.to_string(), frag.to_string());
        // uniform float screen_y_ratio;
        // uniform float zoom;
        // uniform vec2 player_pos;
        let screen_y_ratio = program.get_uniform("screen_y_ratio").unwrap();
        let zoom = program.get_uniform("zoom").unwrap();
        let player_pos = program.get_uniform("player_pos").unwrap();
        Self {
            program,
            dynamic_stack: BufferStack::new(attributes.clone()),
            static_stack: BufferStack::new(attributes),
            screen_y_ratio,
            zoom,
            player_pos
        }
    }

    pub fn submit(
        &mut self,
        builder: VertexBuilder<V>,
        stability: RenderLayerStability,
    ) -> Result<()> {
        let stack = match stability {
            RenderLayerStability::Stable => &mut self.static_stack,
            RenderLayerStability::Erratic => &mut self.dynamic_stack,
        };

        stack.indices.set(&builder.indices)?;
        stack.vertex.set(&builder.data)?;
        debug!("Submitted {:?} {}", stability, type_name::<V>());
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32){
        self.screen_y_ratio.set_value(width as f32 / height as f32);
    }

    pub fn draw(&self, prof: &mut Profiler, view: &Player) {
        self.player_pos.set_value(view.pos);
        self.zoom.set_value(view.zoom);

        self.static_stack.draw(&self.program, prof);
        self.dynamic_stack.draw(&self.program, prof);
    }
}

pub struct BufferStack<V: Clone> {
    pipeline: VertexPipeline,
    vertex: Buffer<V>,
    indices: Buffer<u32>,
}

impl<V: Clone> BufferStack<V> {
    fn new(attributes: Vec<AttributeDescriptor>) -> BufferStack<V> {
        let vertex = Buffer::<V>::create(
            BufferType::Vertex(attributes),
            BufferUsage::Static,
            BufferAccess::Draw,
            36868,
        );
        let indices = Buffer::<u32>::create(
            BufferType::index::<u32>(),
            BufferUsage::Static,
            BufferAccess::Draw,
            13828,
        );

        let mut pipeline = VertexPipeline::new();
        pipeline.bind_buffer(&vertex);
        pipeline.bind_buffer(&indices);

        BufferStack {
            pipeline,
            vertex,
            indices,
        }
    }

    pub fn draw(&self, program: &Program, prof: &mut Profiler) {
        if self.indices.get_elements() > 0 {
            prof.count_draw_call();
        }
        program.draw(
            &self.pipeline,
            0..self.indices.get_elements(),
            DrawMode::Triangle,
        );
    }
}
