#![feature(let_else)]

use std::ffi::c_void;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use glfw::{
	Context, Glfw, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint, WindowMode,
};
use glium::{Frame, Surface, SwapBuffersError};
use glium::backend::Backend;
use image::imageops::FilterType;

use rsa_core::api::Api;
use rsa_core::error::{ContextCompat, Result, WrapErr};
use rsa_input::event::Event;
use rustaria::world::World;
use crate::camera::Camera;

use crate::draw::Drawer;
use crate::event::map_event;

mod ty;
mod draw;
mod mesh_builder;
mod neighbor;
pub mod render;
pub mod camera;
mod event;

pub struct GraphicSystem {
	events: Receiver<(f64, WindowEvent)>,
	width: u32,
	height: u32,

	/// The Drawer contains the opengl stuff needed to render
	drawer: Drawer,
}

impl GraphicSystem {
	pub fn new(width: u32, height: u32) -> Result<GraphicSystem> {
		// Glfw
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
		glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
		glfw.window_hint(WindowHint::ContextVersion(4, 6));

		// Window
		let (mut window, events) = glfw
			.create_window(width, height, "Rustaria", WindowMode::Windowed)
			.wrap_err("Could not create window")?;

		window.make_current();
		glfw.set_swap_interval(SwapInterval::Sync(1));

		// Polling
		window.set_key_polling(true);
		window.set_size_polling(true);
		window.set_scroll_polling(true);
		window.set_mouse_button_polling(true);
		window.set_framebuffer_size_polling(true);

		// Icon
		let icon = image::load_from_memory(include_bytes!("builtin/icon.png"))?;
		window.set_icon(vec![
			icon.resize(16, 16, FilterType::Triangle).to_rgba8(),
			icon.resize(32, 32, FilterType::Triangle).to_rgba8(),
			icon.resize(48, 48, FilterType::Triangle).to_rgba8(),
			icon.to_rgba8(),
		]);

		let internals = Rc::new(Internals { window, glfw });
		let mut drawer = Drawer::new(internals);
		drawer.resize(width, height);
		Ok(GraphicSystem {
			events,
			width,
			height,
			drawer,
		})
	}

	pub fn poll_events(&mut self) -> Vec<Event> {
		unsafe {
			let mut events = Vec::new();
			glfw::ffi::glfwPollEvents();
			while let Ok((_, event)) = self.events.try_recv() {
				match &event {
					WindowEvent::Size(x, y) => {
						self.width = *x as u32;
						self.height = *y as u32;
						self.drawer.resize(self.width, self.height);

					}
					WindowEvent::Scroll(_, y) => {
						self.drawer.camera.scale += *y as f32;
					}
					WindowEvent::Close => {
						glfw::ffi::glfwSetWindowShouldClose(self.drawer.internals.window.window_ptr(), true as i32);
					}
					_ => {}
				}

				if let Some(event) = map_event(event) {
					events.push(event);
				}
			}

			events
		}
	}

	pub fn running(&self) -> bool {
		!self.drawer.internals.window.should_close()
	}

	pub fn start_draw(&mut self, camera: &Camera) -> Draw {
		self.drawer.camera = camera.clone();
		let mut frame = Frame::new(self.drawer.context.clone(), (self.width, self.height));
		frame.clear_color(0.1, 0.1, 0.1, 1.0);

		Draw {
			frame,
			system: self
		}
	}

	pub fn reload(&mut self, api: &Api) -> Result<()> {
		self.drawer.reload(api).wrap_err("Failed to reload drawer")?;
		Ok(())
	}
}

pub struct Draw<'a> {
	pub(crate) frame: Frame,
	pub(crate) system: &'a mut GraphicSystem
}

impl<'a> Draw<'a> {
	pub fn finish(self)  -> Result<()>{
		self.frame.finish()?;
		Ok(())
	}
}


pub(crate) struct Internals {
	window: Window,
	glfw: Glfw,
}

unsafe impl Backend for Internals {
	fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
		unsafe {
			glfw::ffi::glfwSwapBuffers(self.window.window_ptr());
		}
		Ok(())
	}

	unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
		self.glfw.get_proc_address_raw(symbol)
	}

	fn get_framebuffer_dimensions(&self) -> (u32, u32) {
		let size = self.window.get_framebuffer_size();
		(size.0 as u32, size.1 as u32)
	}

	fn is_current(&self) -> bool {
		self.window.is_current()
	}

	unsafe fn make_current(&self) {
		glfw::ffi::glfwMakeContextCurrent(self.window.window_ptr());
	}
}