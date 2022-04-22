use std::{rc::Rc, sync::mpsc::Receiver};

use eyre::Result;
use glfw::{Glfw, Window, WindowEvent};
use glium::{
	texture::{self, RawImage2d, SrgbTexture2d},
	uniform, Frame, Rect, Surface,
};
use image::{imageops::FilterType, DynamicImage};

use engine::{GlfwBackendEngine, GliumBackendEngine};
use pipeline::LayerPipeline;
use rustaria_util::{info, trace};
use rustariac_backend::ty::AtlasLocation;
use rustariac_backend::{
	layer::LayerChannel,
	ty::{Camera, PosTexture},
};

pub mod engine;
mod pipeline;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GliumPosTexture {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

glium::implement_vertex!(GliumPosTexture, position, tex_coords);

pub struct GliumBackend {
	facade: GliumBackendEngine,
	engine: Rc<GlfwBackendEngine>,

	atlas: Option<SrgbTexture2d>,
	pos_texture: LayerPipeline<GliumPosTexture>,
	width: u32,
	height: u32,
}

impl GliumBackend {
	pub fn new(glfw: Glfw, window: Window, events: Receiver<(f64, WindowEvent)>) -> Result<Self> {
		let size = window.get_size();
		let engine = Rc::new(GlfwBackendEngine {
			window,
			events,
			glfw,
		});

		let facade = GliumBackendEngine {
			context: unsafe {
				glium::backend::Context::new(engine.clone(), true, Default::default())
			}?,
		};

		Ok(GliumBackend {
			pos_texture: LayerPipeline::new(
				&facade,
				include_str!("./gl/texture.frag.glsl"),
				include_str!("./gl/texture.vert.glsl"),
			),
			facade,
			engine,
			atlas: None,
			width: size.0 as u32,
			height: size.1 as u32,
		})
	}
}

impl rustariac_backend::Backend for GliumBackend {
	fn window(&self) -> &Window {
		&self.engine.window
	}

	fn glfw(&self) -> &Glfw {
		&self.engine.glfw
	}

	fn supply_atlas(
		&mut self,
		width: u32,
		height: u32,
		images: Vec<(DynamicImage, AtlasLocation)>,
		level: u32,
	) {
		let atlas = texture::SrgbTexture2d::empty_with_mipmaps(
			&self.facade,
			texture::MipmapsOption::EmptyMipmapsMax(level),
			width,
			height,
		)
		.unwrap();

		for level in 0..=level {
			if let Some(mipmap) = atlas.mipmap(level) {
				for (image, location) in &images {
					let left = location.origin.x as u32 >> level;
					let bottom = location.origin.y as u32 >> level;
					let width = location.width() as u32 >> level;
					let height = location.height() as u32 >> level;
					let image = image.resize_exact(width, height, FilterType::Nearest);
					mipmap.write(
						Rect {
							left,
							bottom,
							width,
							height,
						},
						RawImage2d::from_raw_rgba_reversed(image.as_bytes(), (width, height)),
					);
				}
			}
		}

		self.atlas = Some(atlas);
	}

	fn draw(&mut self, camera: &Camera) {
		let mut frame = Frame::new(self.facade.context.clone(), (self.width, self.height));
		frame.clear_color(0.1, 0.1, 0.1, 1.0);
		if let Some(atlas) = &self.atlas {
			let draw_parameters = glium::draw_parameters::DrawParameters {
				blend: glium::draw_parameters::Blend::alpha_blending(),
				..glium::draw_parameters::DrawParameters::default()
			};
			let uniforms = uniform! {
					  screen_y_ratio: self.width as f32 / self.height as f32,
					  zoom: camera.zoom,
					  player_pos: camera.position,
					  tex: glium::uniforms::Sampler::new(atlas)
			.minify_filter(glium::uniforms::MinifySamplerFilter::NearestMipmapNearest)
			.magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)};

			self.pos_texture
				.draw(&self.facade, &mut frame, &uniforms, &draw_parameters);
		} else {
			trace!(target: "draw@rustariac.glium", "Atlas is not loaded. Skipping rendering frame");
		}

		frame.finish().unwrap();
	}

	fn new_layer_pos_tex(&mut self) -> LayerChannel<PosTexture> {
		unsafe {
			// LOL
			std::mem::transmute(self.pos_texture.create_layer(&self.facade))
		}
	}

	fn size(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	fn poll_events(&mut self) -> Vec<WindowEvent> {
		unsafe {
			let mut events = Vec::new();
			glfw::ffi::glfwPollEvents();
			while let Ok((_, event)) = self.engine.events.try_recv() {
				if let WindowEvent::Size(x, y) = &event {
					self.width = *x as u32;
					self.height = *y as u32;
				}

				events.push(event);
			}
			events
		}
	}

	fn mark_dirty(&mut self) {
		self.pos_texture.mark_dirty();
	}
}
