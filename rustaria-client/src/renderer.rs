use std::borrow::Cow;

use bytemuck::Pod;
use naga::ShaderStage;
use wgpu::{Buffer, BufferUsages, CommandBuffer, Device, Face, PrimitiveState, Queue, ShaderModuleDescriptor, ShaderSource, Surface, SurfaceConfiguration, TextureView, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::DeviceExt;
use wgpu::VertexStepMode::Vertex;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::renderer::tile_drawer::TileDrawer;

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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadPos {
    x: f32,
    y: f32,
}

pub trait Drawer {
    fn new(device: &Device, config: &SurfaceConfiguration) -> Self;
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn draw(&mut self, view: &TextureView, device: &Device) -> Result<CommandBuffer, wgpu::SurfaceError> ;
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

        let tile_drawer = TileDrawer::new(&device, &config);
        Self {
            surface,
            device,
            queue,
            config,
            size,
            tile_drawer
        }
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
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());


        self.queue.submit(vec![
            self.tile_drawer.draw(&view, &self.device)?
        ]);

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
