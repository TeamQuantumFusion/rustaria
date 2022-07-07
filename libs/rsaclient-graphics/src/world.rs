use glium::{program::SourceCode, Frame, Program};
use rsa_core::err::ext::AuditExt;

use crate::world::{chunk::WorldChunkRenderer, entity::WorldEntityRenderer};
use rsa_core::err::Result;
use rsa_world::World;
use rsaclient_core::debug::Debug;
use rsaclient_core::frontend::Frontend;
use rsaclient_core::timing::Timing;
use rsaclient_core::ty::{Draw, Viewport};
use rsaclient_player::PlayerSystem;
use crate::GraphicsRPC;

pub mod chunk;
pub mod entity;
pub mod neighbor;

pub struct WorldRenderer {
	pos_color_program: Program,
	chunk_renderer: WorldChunkRenderer,
	entity_renderer: WorldEntityRenderer,
}

impl WorldRenderer {
	pub fn reload(&mut self) { self.chunk_renderer.reload(); }
}

impl WorldRenderer {
	pub fn new(frontend: &Frontend) -> Result<Self> {
		Ok(Self {
			pos_color_program: Program::new(
				&frontend.ctx,
				SourceCode {
					vertex_shader: include_str!("../../rsaclient-core/src/builtin/pos_tex.vert.glsl"),
					tessellation_control_shader: None,
					tessellation_evaluation_shader: None,
					geometry_shader: None,
					fragment_shader: include_str!("../../rsaclient-core/src/builtin/pos_tex.frag.glsl"),
				},
			)?,
			chunk_renderer: WorldChunkRenderer::new()?,
			entity_renderer: WorldEntityRenderer::new(frontend)?,
		})
	}

	pub fn tick(
		&mut self,
		frontend: &Frontend,
		world: &World,
	) -> Result<()> {
		self.chunk_renderer.tick(frontend, &world.chunks)?;
		Ok(())
	}

	pub fn draw(
		&mut self,
		rpc: &GraphicsRPC,
		frontend: &Frontend,
		player: &PlayerSystem,
		world: &World,
		frame: &mut Frame,
		viewport: &Viewport,
		debug: &mut Debug,
		timing: &Timing,
	) -> Result<()> {
		let mut draw = Draw {
			frame,
			viewport,
			atlas: rpc.atlas.as_ref().wrap_err("Atlas is not available")?,
			frontend,
			debug,
			timing,
		};
		self.chunk_renderer
			.draw(rpc, &world.chunks, &self.pos_color_program, &mut draw)?;
		self.entity_renderer.draw(
			rpc,
			player,
			&world.entities,
			&self.pos_color_program,
			&mut draw,
		)?;
		Ok(())
	}
}
