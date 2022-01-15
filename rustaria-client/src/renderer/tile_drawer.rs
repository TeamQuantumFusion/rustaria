use naga::ShaderStage;
use wgpu::{Buffer, BufferUsages, CommandBuffer, Device, FragmentState, RenderPipeline, SurfaceConfiguration, TextureView, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use winit::dpi::PhysicalSize;

use crate::renderer::{create_buffer, DEFAULT_PRIMITIVE, Drawer, get_shader_module, QuadPos};

pub struct TileDrawer {
    // Pipeline
    pipeline: RenderPipeline,
    // Buffers
    tile_buffer: Buffer,
    quad_index_buffer: Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileTexturePos {
    x: u32,
    y: u32,
}

impl Drawer for TileDrawer {
    fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Tile Pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let fragment_module = get_shader_module(
            "triangle-fs",
            include_str!("../shader/triangle-fs.glsl"),
            ShaderStage::Fragment,
        );
        let vertex_module = get_shader_module(
            "triangle-vs",
            include_str!("../shader/triangle-vs.glsl"),
            ShaderStage::Vertex,
        );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Tile Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &device.create_shader_module(&vertex_module),
                entry_point: "main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<(f32, f32)>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            fragment: Some(FragmentState {
                module: &device.create_shader_module(&fragment_module),
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: DEFAULT_PRIMITIVE,
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let tile_buffer = create_buffer(
            device,
            "stuff",
            &[
                QuadPos { x: -0.5, y: 0.5 },
                QuadPos { x: -0.5, y: -0.5 },
                QuadPos { x: 0.5, y: 0.5 },
                QuadPos { x: 0.5, y: -0.5 },
            ],
            BufferUsages::VERTEX,
        );

        let quad_index_buffer = create_buffer(
            device,
            "Quad Index Buffer",
            &[0u16, 1u16, 2u16, 2u16, 1u16, 3u16],
            BufferUsages::INDEX,
        );

        TileDrawer {
            pipeline,
            tile_buffer,
            quad_index_buffer,
        }
    }

    fn resize(&mut self, _: PhysicalSize<u32>) {}

    fn draw(&mut self, view: &TextureView, device: &Device) -> Result<CommandBuffer, wgpu::SurfaceError> {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder"), });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.41,
                        g: 0.57,
                        b: 0.97,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.tile_buffer.slice(..));
        render_pass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..(24 * 24));

        // drop render pass here because it mutably borrows `encoder`,
        // and we wanna use it later
        drop(render_pass);
        Ok(encoder.finish())
    }
}