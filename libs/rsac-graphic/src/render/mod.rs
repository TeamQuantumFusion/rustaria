use crate::render::chunk::ChunkRenderer;
use crate::{Draw, GraphicSystem};
use glium::{Program};
use glium::program::{SourceCode};
use rsa_core::api::Api;
use rsa_core::error::Result;
use rsa_core::ty::ChunkPos;
use rustaria::world::World;

mod chunk;
mod entity;

pub struct WorldRenderer {
	chunk: ChunkRenderer,
}

impl WorldRenderer {
	pub fn new(system: &mut GraphicSystem) -> Result<WorldRenderer> {
		system.drawer.load_program("pos_tex", Program::new(&system.drawer.context, SourceCode {
			vertex_shader: include_str!("../builtin/shader/pos_tex.vert.glsl"),
			tessellation_control_shader: None,
			tessellation_evaluation_shader: None,
			geometry_shader: None,
			fragment_shader: include_str!("../builtin/shader/pos_tex.frag.glsl")
		})?);

		Ok(WorldRenderer {
			chunk: ChunkRenderer::new(&system.drawer)?,
		})
	}

	pub fn notify_chunk(&mut self, pos: ChunkPos) {
		self.chunk.dirty_chunk(pos);
	}

	pub fn draw(&mut self, draw: &mut Draw, world: &World) -> Result<()> {
		self.chunk.draw(draw, &world.chunks)?;
		Ok(())
	}

	pub fn reload(&mut self, api: &Api, graphics: &mut GraphicSystem) -> Result<()>{
		self.chunk.reload(api, &graphics.drawer);
		Ok(())
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
