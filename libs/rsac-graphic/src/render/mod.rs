use crate::render::chunk::ChunkRenderer;
use crate::Drawer;
use glium::{Frame, Program};
use glium::program::{SourceCode};
use rsa_core::api::Api;
use rsa_core::error::Result;
use rsa_core::ty::ChunkPos;
use rustaria::world::World;

mod chunk;

pub struct WorldRenderer {
	chunk: ChunkRenderer,
}

impl WorldRenderer {
	pub fn notify_chunk(&mut self, pos: ChunkPos) {
		self.chunk.dirty_chunk(pos);
	}

	pub(crate) fn new(drawer: &mut Drawer) -> Result<WorldRenderer> {
		drawer.load_program("pos_tex",Program::new(&drawer.context, SourceCode {
			vertex_shader: include_str!("../builtin/shader/pos_tex.vert.glsl"),
			tessellation_control_shader: None,
			tessellation_evaluation_shader: None,
			geometry_shader: None,
			fragment_shader: include_str!("../builtin/shader/pos_tex.frag.glsl")
		})?);

		Ok(WorldRenderer {
			chunk: ChunkRenderer::new(drawer)?,
		})
	}

	pub(crate) fn draw(&mut self, frame: &mut Frame, drawer: &Drawer, world: &World) -> Result<()> {
		self.chunk.draw(frame, drawer, &world.chunks)?;
		Ok(())
	}

	pub(crate) fn reload(&mut self, api: &Api, drawer: &Drawer) {
		self.chunk.reload(api, drawer);
	}
}

fn variation(x: u32, y: u32) -> u32 {
	let offset_x = x.overflowing_add(69420).0.overflowing_mul(69).0;
	let mut v = offset_x.overflowing_mul(y + 420).0;
	v ^= v.overflowing_shl(13).0;
	v ^= v.overflowing_shr(7).0;
	v ^= v.overflowing_shl(17).0;
	v
}
