use std::sync::Arc;
use rsa_core::api::Api;
use rsa_core::error::{Context, Report, Result};
use rsa_core::logging::{debug, info};
use rsa_core::reload;
use rsa_core::settings::UPS;
use rsac_graphic::GraphicSystem;
use rustaria::chunk::layer::tile::TilePrototype;
use rustaria::entity::prototype::EntityPrototype;
use std::time::{Duration, Instant};
use rayon::{ThreadPool, ThreadPoolBuilder};
use rsa_core::math::vec2;
use rsac_graphic::camera::Camera;
use rustaria::CarrierUnavailable;

use crate::args::{ClientMode, ClientOptions};
use crate::input::InputModule;
use crate::player::PlayerModule;
use crate::world::ClientWorld;

mod args;
mod world;
mod input;
mod player;

const TICK_DURATION: Duration = Duration::from_nanos((1000000000 / UPS) as u64);

fn main() -> Result<()> {
	let mut client = Client::new().wrap_err("Failed to initialize core systems")?;
	client.reload()?;
	client.world = Some(ClientWorld::new_integrated(&client.api, &mut client.graphics,client.thread_pool.clone())?);

	// If the loop fails we try to recover it. Else we nuke the client-old and go on with our day.
	while let Err(report) = client.run_loop() {
		client.try_recover(report)?;
	}
	Ok(())
}

pub struct Client {
	api: Api,
	thread_pool: Arc<ThreadPool>,
	camera: Camera,
	options: ClientOptions,

	world: Option<ClientWorld>,

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

		// Api initialization
		info!(target: "init@rustaria", "Initializing Api");
		let api = Api::new(
			dir.join("plugins"),
			match options.mode {
				ClientMode::KernelDev => vec![dir.join("../../../plugin")],
				_ => vec![],
			},
		)?;

		let input = InputModule::new();

		Ok(Client {
			options,
			camera: Camera {
				pos: vec2(8.0, 8.0),
				scale: 20.0
			},
			api,
			input,
			graphics,
			world: None,
			thread_pool: Arc::new(ThreadPoolBuilder::new().build()?)
		})
	}

	pub fn reload(&mut self) -> Result<()> {
		reload!((TilePrototype, EntityPrototype) => &mut self.api);
		self.graphics
			.reload(&self.api)
			.wrap_err("Failed to reload Graphics System")?;

		if let Some(world) = &mut self.world {
			world.reload(&self.api, &mut self.graphics)?;
		}
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

			// TODO tick_ops

			let tick_pos = Instant::now().checked_duration_since(last_tick).unwrap().as_secs_f32() / TICK_DURATION.as_secs_f32();

			self.draw(tick_pos).map_err(LoopError::DrawFail)?;
		}

		Ok(())
	}

	pub fn tick_input(&mut self) -> Result<()> {
		self.input.tick_input(self.graphics.poll_events());
		Ok(())
	}

	/// Called every update, Used for communicating with the server as it runs on this update speed.
	pub fn tick(&mut self) -> Result<()> {
		if let Some(world) = &mut self.world {
			world.tick(&mut self.input)?;
		}
		Ok(())
	}

	/// Called every frame, Used for drawing stuff on screen.
	pub fn draw(&mut self, tick_pos: f32) -> Result<()> {
		self.input.setup_camera(&mut self.camera);
		if let Some(world) = &mut self.world  {
			world.setup_camera(&mut self.camera, tick_pos)?;
		}

		let mut draw = self.graphics.start_draw(&self.camera);
		if let Some(world) = &mut self.world  {
			world.draw(&mut draw)?;
		}
		draw.finish()?;
		Ok(())
	}

	pub fn try_recover(&mut self, error: LoopError) -> Result<()> {
		if let LoopError::ServerFail(report) = &error {
			if report.downcast_ref::<CarrierUnavailable>().is_some() {
				debug!("{report}");
				return self.reload().wrap_err("Failed to reload on bail.");
			} else {
				//todo!("Leave server")
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
