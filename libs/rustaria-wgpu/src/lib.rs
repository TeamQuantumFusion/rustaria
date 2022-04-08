extern crate glfw;

use std::borrow::Cow;

use futures::executor::block_on;
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

use rustaria_graphics::vertex::VertexBuilder;
use rustaria_graphics::RenderLayerIdentifier;

pub struct WgpuBackend {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl rustaria_graphics::RenderBackend for WgpuBackend {
    fn new(glfw: &mut Glfw, window: &mut Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&*window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            // Request an adapter which can render to our surface
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .expect("Failed to create device");

        WgpuBackend {
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }

    fn resize(&mut self, size: (u32, u32)) {
        todo!()
    }

    fn submit<V: Clone>(&mut self, identifier: RenderLayerIdentifier, buffer: VertexBuilder<V>) {
        todo!()
    }

    fn draw(&mut self) {}
}
