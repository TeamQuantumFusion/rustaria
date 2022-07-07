#![feature(drain_filter)]
#![allow(clippy::new_without_default)]

extern crate core;

use std::{
	path::PathBuf,
	time::{Instant},
};

use glfw::{Action, Key, WindowEvent};
use glium::Surface;
use rsa_client_core::{
	debug::Debug,
	frontend::Frontend,
	timing::Timing,
	ty::{Viewport},
};
use rsa_core::{
	api::{reload::Reload, stargate::Stargate, Core},
	blake3::Hasher,
	err::{ext::AuditExt, Result},
	initialize,
	math::vec2,
	ty::Identifier,
};
use rsa_world::{
	chunk::{storage::ChunkStorage, Chunk, ChunkLayer},
	ty::ChunkPos,
	World,
};

use crate::{game::ClientGame, rpc::ClientRPC};

mod game;
pub mod rpc;

fn main() -> Result<()> {
	initialize()?;
	let mut client = Client::new()?;
	client.reload().wrap_err("Failed to loafd game.")?;
	client.run()?;
	Ok(())
}

pub struct Client {
	core: Core,

	viewport: Viewport,
	debug: Debug,
	game: Option<ClientGame>,
	rpc: ClientRPC,

	frontend: Frontend,
	reload_requested: bool,
}

impl Client {
	pub fn new() -> Result<Client> {
		let run_dir = std::env::current_dir().wrap_err("Could not find current directory.")?;
		let frontend = Frontend::new().wrap_err("Could not initialize frontend.")?;
		let debug = Debug::new(&frontend).wrap_err("Could not initialize debug render.")?;
		//debug.enable(DebugCategory::TileSpread);
		//debug.enable(DebugCategory::EntityCollision);
		//debug.enable(DebugCategory::EntityVelocity);
		//debug.enable(DebugCategory::ChunkMeshing);
		//debug.enable(DebugCategory::ChunkBorders);
		//
		Ok(Client {
			core: Core::new(run_dir, vec![PathBuf::from("../plugin")])?,
			viewport: Viewport::new(vec2(0.0, 0.0), 1.0),
			debug,
			frontend,
			game: None,
			reload_requested: false,
			rpc: Default::default(),
		})
	}

	pub fn run(&mut self) -> Result<()> {
		let mut timing = Timing::new();
		while self.frontend.running() {
			self.tick_events()?;

			while timing.next_tick() {
				self.tick()?;
			}

			self.draw(&timing)?;
			self.debug.tick();

			if self.reload_requested {
				self.reload().wrap_err("Failed to reload")?;
				timing = Timing::new();
				self.reload_requested = false;
			}
		}

		Ok(())
	}

	pub fn tick_events(&mut self) -> Result<()> {
		let start = Instant::now();
		for event in self.frontend.poll_events() {
			if let WindowEvent::Key(Key::O, _, _, _) = event {
				self.game = Some(self.join_world()?);
			}
			if let WindowEvent::Key(Key::R, _, Action::Press, _) = event {
				self.reload_requested = true;
				if let Some(game) = &mut self.game {
					game.renderer.reload();
				}
			}
			if let Some(world) = &mut self.game {
				world.event(&self.frontend, event);
			}
		}
		self.debug.log_event(start);
		Ok(())
	}

	pub fn tick(&mut self) -> Result<()> {
		// TODO carrier in lua
		//let api_scope = LuaScope::from(&*self.core);
		//self.core.lua.globals().insert("api", api_scope.lua())?;

		let start = Instant::now();
		if let Some(world) = &mut self.game {
			world.tick(
				&self.frontend,
				&self.core,
				&self.rpc,
				&self.viewport,
				&mut self.debug,
			)?
		}
		self.debug.log_tick(start);
		Ok(())
	}

	pub fn draw(&mut self, timing: &Timing) -> Result<()> {
		let start = Instant::now();
		let mut frame = self.frontend.start_draw();
		frame.clear_color(0.10, 0.10, 0.10, 1.0);

		if let Some(world) = &mut self.game {
			if let Some(viewport) = world.get_viewport() {
				self.viewport.pos -= ((self.viewport.pos - viewport.pos) * 0.2) * timing.step();
				//self.viewport.pos = viewport.pos;
				self.viewport.zoom = viewport.zoom;
				self.viewport.recompute_rect(self.frontend.aspect_ratio);
			}

			world.renderer.draw(
				&self.rpc.graphics,
				&self.frontend,
				&world.player,
				&world.world,
				&mut frame,
				&self.viewport,
				&mut self.debug,
				timing,
			)?;
		}
		self.debug.log_draw(start);
		self.debug
			.draw(&self.frontend, &self.viewport, &mut frame)?;
		frame.finish()?;
		Ok(())
	}

	pub fn reload(&mut self) -> Result<()> {
		let mut reload = Reload {
			stargate: Stargate::new(),
			client: true,
		};
		ClientRPC::register(&mut reload.stargate, &self.core.lua)
			.wrap_err("Failed to register ClientRPC")?;
		self.core.reload(&mut reload).wrap_err("Failed to reload")?;
		self.rpc = ClientRPC::build(&self.frontend, &self.core, &mut reload.stargate)
			.wrap_err("Failed to build ClientRPC")?;

		let mut hasher = Hasher::new();
		self.rpc.append_hasher(&mut hasher);
		self.core.hash = Some(hasher.finalize());
		Ok(())
	}

	pub fn join_world(&self) -> Result<ClientGame> {
		let mut storage = ChunkStorage::new(9, 9);

		for y in 0..9 {
			for x in 0..9 {
				storage.insert(
					ChunkPos {
						x: x as u32,
						y: y as u32,
					},
					Chunk {
						layers: self
							.rpc
							.world
							.block_layer
							.table
							.iter()
							.map(|(layer_id, prototype)| {
								let id = prototype
									.blocks
									.get_id_from_identifier(&Identifier::new("dirt"))
									.expect("where dirt");
								let dirt = prototype.blocks.get(id).create(id);

								let id = prototype
									.blocks
									.get_id_from_identifier(&Identifier::new("air"))
									.expect("where air");
								let air = prototype.blocks.get(id).create(id);

								(
									layer_id,
									if x == 2 && y == 1 {
										let a = air;
										let d = dirt;

										ChunkLayer {
											data: [
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, d, d, d, a, d, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, d, d, d, a, d, a, a, a, a, a, a, a, a, a, a],
												[a, d, d, d, a, d, a, a, a, a, a, a, a, a, a, a],
												[a, d, d, d, a, d, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
												[a, a, a, a, a, a, a, a, a, a, a, a, a, a, a, a],
											],
										}
									} else if x == 0 || (y > 0 && x != 2) || x > 3 {
										ChunkLayer::new_copy(air)
									} else {
										ChunkLayer::new_copy(dirt)
									},
								)
							})
							.collect(),
					},
				);
			}
		}
		ClientGame::new_integrated(&self.frontend, &self.rpc, World::new(storage).unwrap())
	}
}
