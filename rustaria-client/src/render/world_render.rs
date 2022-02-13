use glfw::Window;

use opengl_render::atlas::{Atlas, AtlasBuilder};
use opengl_render::attribute::{AttributeDescriptor, AttributeType};
use opengl_render::buffer::{
    Buffer, BufferAccess, BufferType, BufferUsage, DrawMode, VertexBufferLayout,
};
use opengl_render::program::VertexPipeline;
use opengl_render::texture::Sampler2d;
use opengl_render::uniform::Uniform;
use opengl_render::{OpenGlBackend, OpenGlFeature};

use crate::render::texture_format::TileImagePos;

#[repr(C)]
pub struct QuadImageVertex {
    pos: [f32; 2],
    pos_texture: [f32; 2],
}

impl QuadImageVertex {
    pub fn quad(
        atlas: &Atlas<String>,
        data: &mut Vec<QuadImageVertex>,
        x: f32,
        y: f32,
        tile: &str,
        ty: TileImagePos,
    ) {
        let loc = atlas.lookup.get(&tile.to_owned()).unwrap();

        let tile_size = loc.height / 4.0;
        let (o_x, o_y) = ty.get_tex_pos();
        let (t_x, t_y) = (loc.x + (o_x * tile_size), loc.y + (o_y * tile_size));
        // todo custom variant amounts. currently its forced to be 3
        data.push(QuadImageVertex {
            pos: [(x + 1.0) / 100.0, (y + 1.0) / 100.0],
            pos_texture: [t_x + tile_size, t_y],
        });
        data.push(QuadImageVertex {
            pos: [(x + 1.0) / 100.0, (y + 0.0) / 100.0],
            pos_texture: [t_x + tile_size, t_y + tile_size],
        });
        data.push(QuadImageVertex {
            pos: [(x + 0.0) / 100.0, (y + 0.0) / 100.0],
            pos_texture: [t_x, t_y + tile_size],
        });
        data.push(QuadImageVertex {
            pos: [(x + 0.0) / 100.0, (y + 1.0) / 100.0],
            pos_texture: [t_x, t_y],
        });
    }
}

pub struct WorldRenderer {
    qi_atlas: Atlas<String>,
    qi_u_atlas_sampler: Uniform<Sampler2d>,
    qi_atlas_sampler: Sampler2d,
    qi_pipeline: VertexPipeline,
    qi_layout: VertexBufferLayout,
    qi_buffer: Buffer<QuadImageVertex>,
    qi_u_screen_y_ratio: Uniform<f32>,
    pub qi_u_zoom: Uniform<f32>,
    pub qi_pos: Uniform<[f32; 2]>,
    pub qi_index_buffer: Buffer<u16>,
}

impl WorldRenderer {
    pub fn new(backend: &mut OpenGlBackend, window: &Window) -> WorldRenderer {
        backend.enable(OpenGlFeature::Alpha);

        let mut atlas = AtlasBuilder::new();
        for x in std::fs::read_dir("./assets/sprite/tile/").unwrap() {
            let entry = x.unwrap();
            if !entry.path().is_dir() {
                let string = entry.file_name().into_string().unwrap();
                println!("{}", string);
                atlas.push(string, image::open(entry.path()).unwrap());
            }
        }

        let atlas = atlas.export(4);

        let a_pos = AttributeDescriptor::new(0, AttributeType::Float(2));
        let a_tex = AttributeDescriptor::new(1, AttributeType::Float(2));
        let mut pipeline = VertexPipeline::new(
            include_str!("./shader/quad_image.v.glsl").to_string(),
            include_str!("./shader/quad_image.f.glsl").to_string(),
        );

        let mut data = Vec::new();
        for y in 0..24 {
            for x in 0..24 {
                QuadImageVertex::quad(
                    &atlas,
                    &mut data,
                    0.0 + x as f32,
                    0.0 + y as f32,
                    "grass.png",
                    TileImagePos::Solid,
                );
            }
        }

        let index_buffer = Buffer::create_index(vec![0, 1, 3, 1, 2, 3u16], 4, 24 * 24);
        let buffer = Buffer::create(
            BufferType::Vertex(vec![a_pos, a_tex]),
            BufferUsage::Static,
            BufferAccess::Draw,
            Some(&data),
        );

        let mut layout = VertexBufferLayout::new();
        layout.bind_buffer(&buffer);
        layout.bind_index(&index_buffer);

        let uniform = pipeline.get_uniform("atlas").unwrap();
        let sampler = backend.create_sampler(0, &atlas.texture);

        let mut screen_y_ratio = pipeline.get_uniform("screen_y_ratio").unwrap();
        let mut zoom = pipeline.get_uniform("zoom").unwrap();
        let pos = pipeline.get_uniform("pos").unwrap();
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
            qi_pos: pos,
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
                    DrawMode::Line
                } else {
                    DrawMode::Triangle
                }
            });
    }
}
