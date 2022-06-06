use rsa_core::api::Api;
use rsa_core::error::{Context, Report, Result};
use rsa_core::logging::info;
use rsa_core::reload;
use rsa_core::settings::UPS;
use rsac_graphic::GraphicSystem;
use rustaria::chunk::layer::tile::TilePrototype;
use rustaria::entity::prototype::EntityPrototype;
use rustaria::world::World;
use rustaria::RichError;
use std::time::{Duration, Instant};
use rsa_core::math::vec2;
use rsa_core::ty::{ChunkPos, Tag};
use rsac_graphic::camera::Camera;
use rustaria::chunk::Chunk;
use rustaria::chunk::layer::ChunkLayer;

use crate::args::{ClientMode, ClientOptions};
use crate::input::InputModule;

mod args;
mod world;
mod input;

const TICK_DURATION: Duration = Duration::from_nanos((1000000000 / UPS) as u64);

fn main() -> Result<()> {
	let mut client = Client::new().wrap_err("Failed to initialize core systems")?;
	client.reload()?;

	let carrier = client.api.get_carrier();
	let result = carrier.get::<TilePrototype>().create_from_tag(&Tag::rsa("dirt"))?;
	let pos = ChunkPos {
		x: 0,
		y: 0,
	};
	client.world.chunks.put_chunk(pos, Chunk {
		tiles: ChunkLayer::new_copy(result)
	});
	client.graphics.world_renderer.notify_chunk(pos);

	// If the loop fails we try to recover it. Else we nuke the client-old and go on with our day.
	while let Err(report) = client.run_loop() {
		client.try_recover(report)?;
	}
	Ok(())
}

pub struct Client {
	options: ClientOptions,

	world: World,
	camera: Camera,

	api: Api,
	input: InputModule,
	graphics: GraphicSystem,
}

impl Client {
	pub fn new() -> Result<Client> {
		// Kernel core initialization
		let dir = std::env::current_dir().wrap_err("Could not get current directory")?;
		let options = ClientOptions::new();
		rsa_core::initialize(options.logging)?;
		info!(target: "init@rustaria", "Hi there! Im here to say that the rustaria core has initialized!");

		// Graphics initialization
		info!(target: "init@rustaria", "Initializing Graphics");
		let graphics = GraphicSystem::new(900, 600)?;

		let input = InputModule::new();

		// Api initialization
		info!(target: "init@rustaria", "Initializing Api");
		let api = Api::new(
			dir.join("plugins"),
			match options.mode {
				ClientMode::KernelDev => vec![dir.join("../../../plugin")],
				_ => vec![],
			},
		)?;

		Ok(Client {
			options,
			world: World::new(),
			camera: Camera {
				pos: vec2(8.0, 8.0),
				scale: 20.0
			},
			api,
			input,
			graphics,
		})
	}

	pub fn reload(&mut self) -> Result<()> {
		reload!((TilePrototype, EntityPrototype) => &mut self.api);
		self.graphics
			.reload(&self.api)
			.wrap_err("Failed to reload Graphics System")?;
		Ok(())
	}

	pub fn run_loop(&mut self) -> Result<(), LoopError> {
		let mut last_tick = Instant::now();
		while self.graphics.running() {
			self.tick_input().map_err(LoopError::InputFail)?;

			while let Some(value) = Instant::now().checked_duration_since(last_tick) {
				if value >= TICK_DURATION {
					self.tick().map_err(LoopError::ServerFail)?;
					last_tick += TICK_DURATION;
				} else {
					break;
				}
			}

			self.draw().map_err(LoopError::DrawFail)?;
		}

		Ok(())
	}

	pub fn tick_input(&mut self) -> Result<()> {
		for event in self.graphics.poll_events() {
			self.input.system.notify_event(event);
		}
		Ok(())
	}

	/// Called every update, Used for communicating with the server as it runs on this update speed.
	pub fn tick(&mut self) -> Result<()> {
		self.input.tick();
		Ok(())
	}

	/// Called every frame, Used for drawing stuff on screen.
	pub fn draw(&mut self) -> Result<()> {
		self.input.apply_zoom(&mut self.camera);
		self.graphics.draw( &self.camera, &self.world)?;
		Ok(())
	}

	pub fn try_recover(&mut self, error: LoopError) -> Result<()> {
		if let LoopError::ServerFail(report) = &error {
			if let Some(RichError::CarrierUnavailable) = report.downcast_ref::<RichError>() {
				return self.reload().wrap_err("Failed to reload on bail.");
			} else {
				todo!("Leave server")
			}
		}

		Err(match error {
			LoopError::InputFail(result) => result,
			LoopError::ServerFail(result) => result,
			LoopError::DrawFail(result) => result,
		})
	}
}

pub enum LoopError {
	InputFail(Report),
	ServerFail(Report),
	DrawFail(Report),
}
