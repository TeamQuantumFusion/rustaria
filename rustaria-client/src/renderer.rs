use std::borrow::Cow;

use bytemuck::Pod;
use naga::ShaderStage;
use tracing::debug;
use wgpu::util::DeviceExt;
use wgpu::{AdapterInfo, Backend, BindGroup, BindGroupLayout, Buffer, BufferUsages, CommandBuffer, Device, Face, PrimitiveState, Queue, ShaderModuleDescriptor, ShaderSource, SurfaceConfiguration, TextureView};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use rustaria::api::RustariaApi;

use crate::renderer::tile_drawer::TileDrawer;

mod atlas;
mod tile_drawer;

const DEFAULT_PRIMITIVE: PrimitiveState = PrimitiveState {
    topology: wgpu::PrimitiveTopology::TriangleList,
    strip_index_format: None,
    front_face: wgpu::FrontFace::Ccw,
    cull_mode: Some(Face::Back),
    polygon_mode: wgpu::PolygonMode::Fill,
    unclipped_depth: false,
    conservative: false,
};

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    tile_drawer: TileDrawer,
    y_ratio_buffer: Buffer,
    y_ratio_bind_group: BindGroup,
    y_ratio_bind_group_layout: BindGroupLayout,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadPos {
    x: f32,
    y: f32,
}

impl Renderer {
    pub async fn new(window: &Window, api: &mut RustariaApi<'_>) -> Self {
        let mut shader_dir = std::env::current_dir().unwrap();
        shader_dir.push("shaders");

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::from(Backend::Dx12));
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

        let y_ratio_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("y_ratio_buffer"),
                contents: bytemuck::cast_slice(&[size.width as f32 / size.height as f32]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let y_ratio_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("y_ratio_bind_group_layout"),
        });

        let y_ratio_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &y_ratio_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: y_ratio_buffer.as_entire_binding(),
                }
            ],
            label: Some("y_ratio_bind_group"),
        });

        let tile_drawer = TileDrawer::new(&queue, &device, &config, api);

        Self::print_adapter_info(adapter.get_info());



        Self {
            surface,
            device,
            queue,
            config,
            size,
            tile_drawer,
            y_ratio_buffer,
            y_ratio_bind_group,
            y_ratio_bind_group_layout
        }
    }

    fn print_adapter_info(info: AdapterInfo) {
        debug!("Rustaria Client Rendering System Report.");
        debug!("Running {:?} \"{}\"", info.device_type, info.name);
        debug!("With {:?} Backend", info.backend);

        let vendor_guess = match info.vendor {
            0x1002 => "(AMD) ",
            0x10DE => "(Nvidia) ",
            0x8086 => "(Intel) ",
            0x1010 => "(ImgTec) ",
            0x13B5 => "(arm) ",
            0x5143 => "(Qualcomm) ",
            _ => "",
        };
        debug!(
            "Vendor ID {} {}Device ID {}",
            info.vendor, vendor_guess, info.device
        )
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.tile_drawer.resize(new_size);
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.queue
            .submit(vec![self.tile_drawer.draw(&view, &self.device)?]);


        output.present();
        Ok(())
    }
}

pub fn get_shader_module<'a>(
    name: &'static str,
    code: &'static str,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor {
        label: Some(name),
        source: ShaderSource::Wgsl(Cow::from(code)),
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
