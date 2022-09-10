use glium::Frame;

use crate::{
	atlas::Atlas, debug::Debug, frontend::Frontend, timing::Timing, ty::viewport::Viewport,
};

pub struct Draw<'frame, 'camera, 'atlas, 'frontend, 'debug, 'timing> {
	pub frame: &'frame mut Frame,
	pub viewport: &'camera Viewport,
	pub atlas: &'atlas Atlas,
	pub frontend: &'frontend Frontend,
	pub debug: &'debug mut Debug,
	pub timing: &'timing Timing,
}
