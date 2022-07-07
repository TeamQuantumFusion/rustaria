use glium::Frame;

use crate::{debug::Debug, frontend::Frontend, ty::viewport::Viewport, timing::Timing};
use crate::atlas::Atlas;

pub struct Draw<'frame, 'camera, 'atlas, 'frontend, 'debug, 'timing> {
	pub frame: &'frame mut Frame,
	pub viewport: &'camera Viewport,
	pub atlas: &'atlas Atlas,
	pub frontend: &'frontend Frontend,
	pub debug: &'debug mut Debug,
	pub timing: &'timing Timing,
}
