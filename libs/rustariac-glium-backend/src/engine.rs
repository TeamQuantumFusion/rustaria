use std::{ffi::c_void, rc::Rc, sync::mpsc::Receiver};

use glfw::{Context, Glfw, Window, WindowEvent};
use glium::{
	backend::{Backend, Facade},
	SwapBuffersError,
};

pub struct GlfwBackendEngine {
	pub(crate) window: Window,
	pub(crate) events: Receiver<(f64, WindowEvent)>,
	pub(crate) glfw: Glfw,
}

unsafe impl Backend for GlfwBackendEngine {
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

#[derive(Clone)]
pub struct GliumBackendEngine {
	pub(crate) context: Rc<glium::backend::Context>,
}

impl Facade for GliumBackendEngine {
	fn get_context(&self) -> &Rc<glium::backend::Context> {
		&self.context
	}
}
