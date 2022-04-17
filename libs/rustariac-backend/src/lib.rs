use std::collections::HashSet;
use std::sync::{mpsc::Receiver, Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ty::AtlasLocation;
use atlas::Atlas;
use eyre::{ContextCompat, Result};
use glfw::{
	Context, Glfw, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint, WindowMode,
};
use image::DynamicImage;
use layer::LayerChannel;
use rustaria_api::{ty::Tag, Api};
use ty::{Camera, PosTexture};

pub mod atlas;
pub mod builder;
pub mod layer;
pub mod ty;

#[derive(Clone)]
pub struct ClientBackend {
	internals: Arc<RwLock<Internals>>,
}

impl ClientBackend {
	pub fn new<
		B: Backend + 'static,
		F: FnOnce(Glfw, Window, Receiver<(f64, WindowEvent)>) -> Result<B>,
	>(
		func: F,
	) -> Result<ClientBackend> {
		Ok(ClientBackend {
			internals: Arc::new(RwLock::new(Internals::new(func)?)),
		})
	}

	pub fn instance(&self) -> RwLockReadGuard<'_, Internals> {
		self.internals.read().unwrap()
	}

	pub fn instance_mut(&self) -> RwLockWriteGuard<'_, Internals> {
		self.internals.write().unwrap()
	}

	pub fn screen_y_ratio(&self) -> f32 {
		let size = self.instance().backend.size();
		size.0 as f32 / size.1 as f32
	}
}

pub struct Internals {
	pub backend: Box<dyn Backend>,
	pub atlas: Atlas,
}

impl Internals {
	pub fn new<
		B: Backend + 'static,
		F: FnOnce(Glfw, Window, Receiver<(f64, WindowEvent)>) -> Result<B>,
	>(
		func: F,
	) -> Result<Internals> {
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
		glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
		glfw.window_hint(WindowHint::ContextVersion(4, 6));

		let size = (1920 / 2, 1080 / 2);
		let (mut window, events) = glfw
			.create_window(size.0, size.1, "Rustaria", WindowMode::Windowed)
			.wrap_err("Could not create window")?;

		window.make_current();
		window.set_key_polling(true);
		window.set_mouse_button_polling(true);
		window.set_scroll_polling(true);
		window.set_size_polling(true);
		window.set_framebuffer_size_polling(true);
		glfw.set_swap_interval(SwapInterval::Sync(1));
		window.set_size(1920 / 2, 1080 / 2);

		Ok(Internals {
			backend: Box::new(func(glfw, window, events)?),
			atlas: Atlas::default(),
		})
	}

	pub fn supply_atlas(&mut self, api: &Api, sprites: HashSet<Tag>) {
		let (atlas, images) = atlas::build_atlas(api, sprites);
		self.backend
			.supply_atlas(atlas.get_width(), atlas.get_height(), images, 3);
		self.atlas = atlas;
	}
}

pub trait Backend {
	fn window(&self) -> &Window;
	fn glfw(&self) -> &Glfw;
	fn size(&self) -> (u32, u32);

	fn poll_events(&mut self) -> Vec<WindowEvent>;
	fn new_layer_pos_tex(&mut self) -> LayerChannel<PosTexture>;
	fn supply_atlas(
		&mut self,
		width: u32,
		height: u32,
		images: Vec<(DynamicImage, AtlasLocation)>,
		level: u32,
	);
	fn draw(&mut self, camera: &Camera);
}
