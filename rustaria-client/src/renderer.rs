use std::borrow::Cow;

use bytemuck::Pod;
use naga::ShaderStage;
use wgpu::{Buffer, BufferUsages, Device, Face, ShaderModuleDescriptor, ShaderSource, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    buffer: Buffer,
    index_buffer: Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadPos {
    x: f32,
    y: f32,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let mut shader_dir = std::env::current_dir().unwrap();
        shader_dir.push("shaders");

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let fragment_module = get_shader_module(
            "triangle-fs",
            include_str!("shader/triangle-fs.glsl"),
            ShaderStage::Fragment,
        );
        let vertex_module = get_shader_module(
            "triangle-vs",
            include_str!("shader/triangle-vs.glsl"),
            ShaderStage::Vertex,
        );
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
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
            fragment: Some(wgpu::FragmentState {
                module: &device.create_shader_module(&fragment_module),
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let buffer = create_buffer(
            &device,
            "stuff",
            &[
                QuadPos { x: -0.5, y: 0.5 },
                QuadPos { x: -0.5, y: -0.5 },
                QuadPos { x: 0.5, y: 0.5 },
                QuadPos { x: 0.5, y: -0.5 },
            ],
            BufferUsages::VERTEX,
        );

        let index_buffer = create_buffer(
            &device,
            "Quad Index Buffer",
            &[0u16, 1u16, 2u16, 2u16, 1u16, 3u16],
            BufferUsages::INDEX,
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            buffer,
            index_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..(24 * 24));

        // drop render pass here because it mutably borrows `encoder`,
        // and we wanna use it later
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

pub fn get_shader_module<'a>(
    name: &'static str,
    code: &'static str,
    stage: ShaderStage,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor {
        label: Some(name),
        source: ShaderSource::Glsl {
            shader: Cow::from(code),
            stage,
            defines: Default::default(),
        },
    }
}

pub fn create_buffer<V: Pod>(
    device: &Device,
    label: &str,
    contents: &[V],
    usage: BufferUsages,
) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(contents),
        usage,
    })
}
