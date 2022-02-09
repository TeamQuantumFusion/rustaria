use glfw::Window;

use opengl_render::attribute::{AttributeDescriptor, AttributeType};
use opengl_render::buffer::{
    Buffer, BufferAccess, BufferType, BufferUsage, DrawMode, VertexBufferLayout,
};
use opengl_render::program::VertexPipeline;
use opengl_render::texture::{
    FilterType, InternalFormat, Sampler2d, Texture, TextureData, TextureDataFormat,
    TextureDescriptor, TextureMagFilter, TextureType, USampler2d,
};
use opengl_render::uniform::Uniform;
use opengl_render::{OpenGlBackend, OpenGlFeature};

macro_rules! vertex {
    ($(pos: [$X:literal, $Y:literal] tex: [$XT:literal, $YT:literal]),*) => {
        vec![
            $(
             QuadImageVertex {
             pos: [$X + 0.0, $Y + 0.0],
             pos_texture: [$XT  / 12.0 , ($YT)  / 4.0],
         },
         QuadImageVertex {
             pos: [$X + 0.0, $Y + 1.0],
             pos_texture: [($XT + 1.0)  / 12.0, ($YT)  / 4.0],
         },
         QuadImageVertex {
             pos: [$X + 1.0,$Y + 1.0],
             pos_texture: [($XT + 1.0)  / 12.0, ($YT + 1.0) / 4.0],
         },
         QuadImageVertex {
             pos: [$X + 1.0,$Y + 0.0],
             pos_texture: [$XT / 12.0, ($YT + 1.0) / 4.0],
         }
            ),*
        ]
    };
}

#[repr(C)]
pub struct QuadImageVertex {
    pos: [f32; 2],
    pos_texture: [f32; 2],
}

pub struct WorldRenderer {
    qi_atlas: Texture,
    qi_u_atlas_sampler: Uniform<Sampler2d>,
    qi_atlas_sampler: Sampler2d,
    qi_pipeline: VertexPipeline,
    qi_layout: VertexBufferLayout,
    qi_buffer: Buffer<QuadImageVertex>,
    qi_u_screen_y_ratio: Uniform<f32>,
    qi_u_zoom: Uniform<f32>,
    qi_index_buffer: Buffer<u16>,
}

impl WorldRenderer {
    pub fn new(backend: &mut OpenGlBackend, window: &Window) -> WorldRenderer {
        backend.enable(OpenGlFeature::Alpha);

        let a_pos = AttributeDescriptor::new(0, AttributeType::Float(2));
        let a_tex = AttributeDescriptor::new(1, AttributeType::Float(2));
        let mut pipeline = VertexPipeline::new(
            include_str!("./shader/quad_image.v.glsl").to_string(),
            include_str!("./shader/quad_image.f.glsl").to_string(),
        );

        let index_buffer = Buffer::create_index(vec![0, 1, 2, 0, 2, 3u16], 4, 3);
        let buffer = Buffer::create(
            BufferType::Vertex(vec![a_pos, a_tex]),
            BufferUsage::Static,
            BufferAccess::Draw,
            Some(&vertex!(
            pos: [1.0, 0.0] tex: [0.0, 1.0],
            pos: [1.0, 1.0] tex: [1.0, 1.0],
            pos: [0.0, 0.0] tex: [1.0, 0.0]
            )),
        );

        let mut layout = VertexBufferLayout::new();
        layout.bind_buffer(&buffer);
        layout.bind_index(&index_buffer);

        let image = image::open(
            "/home/alphasucks/CLionProjects/rustaria-main/run/assets/sprite/tile/grass.png",
        )
        .unwrap();
        let atlas = Texture::new::<u8>(
            TextureType::Texture2d {
                images: Some(vec![TextureData {
                    texture_data: Vec::from(image.as_bytes()),
                    texture_format: TextureDataFormat::Rgba,
                }]),
                internal: InternalFormat::Rgba,
                width: image.width(),
                height: image.height(),
                border: 0,
            },
            TextureDescriptor {
                mag_filter: TextureMagFilter(FilterType::Nearest),
                ..TextureDescriptor::default()
            },
        );
        let uniform = pipeline.get_uniform("atlas").unwrap();
        let sampler = backend.create_sampler(0, &atlas);

        let mut screen_y_ratio = pipeline.get_uniform("screen_y_ratio").unwrap();
        let mut zoom = pipeline.get_uniform("zoom").unwrap();
        let (width, height) = window.get_size();
        screen_y_ratio.set_value(width as f32 / height as f32);
        zoom.set_value(0.1f32);
        WorldRenderer {
            qi_atlas: atlas,
            qi_u_atlas_sampler: uniform,
            qi_atlas_sampler: sampler,
            qi_pipeline: pipeline,
            qi_layout: layout,
            qi_buffer: buffer,
            qi_u_screen_y_ratio: screen_y_ratio,
            qi_u_zoom: zoom,
            qi_index_buffer: index_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.qi_u_screen_y_ratio
            .set_value(width as f32 / height as f32);
    }

    pub fn draw(&self, wireframe: bool) {
        self.qi_pipeline
            .draw(&self.qi_layout, 0..self.qi_index_buffer.get_size(), {
                if wireframe {
                    DrawMode::LineLoop
                } else {
                    DrawMode::Triangle
                }
            });
    }
}
