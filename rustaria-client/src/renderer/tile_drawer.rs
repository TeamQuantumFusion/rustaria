use eyre::eyre;
use image::RgbaImage;
use rustaria::api::plugin::ArchivePath;
use std::path::PathBuf;
use tracing::{debug, error};
use wgpu::{
    Buffer, BufferUsages, CommandBuffer, Device, FragmentState, Queue, RenderPipeline,
    SurfaceConfiguration, TextureView, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};
use winit::dpi::PhysicalSize;

use rustaria::api::Rustaria;
use rustaria::registry::{RawId, Tag};

use crate::renderer::atlas::Atlas;
use crate::renderer::{create_buffer, get_shader_module, DEFAULT_PRIMITIVE};

pub struct TileDrawer {
    // Pipeline
    pipeline: RenderPipeline,
    // Buffers
    tile_buffer: Buffer,
    quad_index_buffer: Buffer,
    atlas: Atlas<RawId>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileVertex {
    pos: [f32; 2],
    tex_pos: [f32; 2],
}

impl TileDrawer {
    pub fn new(
        queue: &Queue,
        device: &Device,
        config: &SurfaceConfiguration,
        api: &Rustaria<'_>,
    ) -> Self {
        fn read_image(
            id: usize,
            tag: &Tag,
            api: &Rustaria<'_>,
        ) -> eyre::Result<(RawId, RgbaImage)> {
            let archive = api
                .get_plugin_assets(&tag.plugin_id)
                .ok_or_else(|| eyre!("Could not find plugin {}", tag.plugin_id))?;

            let data = archive.get_asset(&ArchivePath::Asset(PathBuf::from(&tag.name)))?;
            let image = image::load_from_memory(data.as_slice())?;
            Ok((id as RawId, image.into_rgba8()))
        }

        let map: Vec<_> = api
            .tiles
            .entries()
            .iter()
            .enumerate()
            .map(|(id, prototype)| (id, prototype.sprite.clone()))
            .collect();

        error!("{:?}", map);

        let images = map
            .into_iter()
            .filter_map(|(id, tile)| match read_image(id, &tile?, api) {
                Ok(image) => Some(image),
                Err(err) => {
                    debug!("Tile Image Skipped {}", err);
                    None
                }
            })
            .collect();

        let atlas = Atlas::new(queue, device, images);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Tile Pipeline layout"),
                bind_group_layouts: &[&atlas.bind_layout],
                push_constant_ranges: &[],
            });

        let shader_module = get_shader_module("tile_module", include_str!("../shader/tile.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Tile Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &device.create_shader_module(&shader_module),
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<TileVertex>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &device.create_shader_module(&shader_module),
                entry_point: "fs_main",
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
            "tile_buffer",
            &[
                TileVertex {
                    pos: [-0.5, 0.5],
                    tex_pos: [0.0, 1.0],
                },
                TileVertex {
                    pos: [-0.5, -0.5],
                    tex_pos: [0.0, 0.0],
                },
                TileVertex {
                    pos: [0.5, 0.5],
                    tex_pos: [1.0, 1.0],
                },
                TileVertex {
                    pos: [0.5, -0.5],
                    tex_pos: [1.0, 0.0],
                },
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
            atlas,
        }
    }

    pub fn resize(&mut self, _: PhysicalSize<u32>) {}

    pub fn draw(
        &mut self,
        view: &TextureView,
        device: &Device,
    ) -> Result<CommandBuffer, wgpu::SurfaceError> {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

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
        render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.tile_buffer.slice(..));
        render_pass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);

        // drop render pass here because it mutably borrows `encoder`,
        // and we wanna use it later
        drop(render_pass);
        Ok(encoder.finish())
    }
}
