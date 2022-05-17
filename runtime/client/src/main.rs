use std::collections::HashSet;
use std::ops::AddAssign;
use std::sync::Arc;
use std::time::{Duration, Instant};

use clap::Parser;
use glfw::{ffi, Action, Key, Modifiers, WindowEvent};
use rayon::{ThreadPool, ThreadPoolBuilder};

use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::packet::entity::ClientEntityPacket;
use rustaria::packet::player::ClientPlayerPacket;
use rustaria::packet::{ClientPacket, PlayerJoinData};
use rustaria::player::Player;
pub use rustaria::prototypes;
pub use rustaria::pt;
use rustaria::SmartError;
use rustaria::{Server};
use rustaria_api::ty::{Prototype, Tag};
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_common::error::Result;
use rustaria_common::logging::debug;
use rustaria_common::math::vec2;
use rustaria_common::settings::UPS;
use rustariac_backend::ty::Camera;
use rustariac_backend::ClientBackend;
use rustariac_glium_backend::GliumBackend;
use world::ClientWorld;

use crate::args::Args;
use crate::internal::chunk::ChunkHandler;
use crate::internal::controller::ControllerHandler;
use crate::internal::entity::EntityHandler;
use crate::internal::rendering::RenderingHandler;

mod args;
mod internal;
mod world;

const DEBUG_MOD: Modifiers =
	Modifiers::from_bits_truncate(ffi::MOD_ALT + ffi::MOD_CONTROL + ffi::MOD_SHIFT);
const UPDATE_TIME: Duration = Duration::from_micros((1000000 / UPS) as u64);

fn main() -> Result<()> {
	let args = args::Args::parse();
	rustaria_common::initialize()?;

	let mut client = Client::new(args)?;
	client.join_integrated()?;

	{
		let lock = client.carrier.lock();
		let prototype = lock.get_registry::<EntityPrototype>();
		let id = prototype
			.id_from_tag(&Tag::new("rustaria:bunne".to_string()).unwrap())
			.unwrap();
		let world = client.world.as_mut().unwrap();
		let pos = vec2(5.0, 5.0);

		world
			.networking
			.send(ClientPacket::Entity(ClientEntityPacket::Spawn(id, pos)))
			.unwrap();
	}
	client.run();

	Ok(())
}

pub struct Client {
	api: Api,
	carrier: Carrier,
	camera: Camera,

	control: ControllerHandler,
	rendering: RenderingHandler,
	world: Option<ClientWorld>,

	// just for house keeping
	thread_pool: Arc<ThreadPool>,
	backend: ClientBackend,
}

impl Client {
	pub fn new(args: Args) -> Result<Client> {
		let backend = ClientBackend::new(GliumBackend::new)?;

		let carrier = Carrier::default();
		let mut dir = std::env::current_dir()?;
		dir.push("plugins");
		let api = Api::new(dir, args.extra_plugin_paths)?;

		let mut client = Client {
			api,
			carrier,
			world: None,
			control: ControllerHandler::new(),
			camera: Camera {
				position: [0.0, 0.0],
				velocity: [0.0, 0.0],
				zoom: 30.0,
				screen_y_ratio: 0.0,
			},
			rendering: RenderingHandler {
				backend: backend.clone(),
			},
			thread_pool: Arc::new(ThreadPoolBuilder::new().num_threads(12).build().unwrap()),
			backend,
		};

		client.reload()?;

		Ok(client)
	}

	pub fn run(&mut self) {
		let mut last_tick = Instant::now();

		let mut reload = false;
		while !self.backend.instance().backend.window().should_close() {
			{
				let mut guard = self.backend.instance_mut();
				for event in guard.backend.poll_events() {
					match event {
						WindowEvent::Size(width, height) => {
							self.camera.screen_y_ratio = width as f32 / height as f32;
						}
						WindowEvent::Scroll(_, y) => {
							self.camera.zoom += y as f32;
						}
						// Reload
						WindowEvent::Key(Key::R, _, Action::Release, DEBUG_MOD) => {
							reload = true;
						}
						// Re-mesh
						WindowEvent::Key(Key::M, _, Action::Release, DEBUG_MOD) => {
							guard.backend.mark_dirty();
						}
						_ => {}
					}

					self.control.consume_event(event);
				}
			}

			while last_tick.elapsed() >= UPDATE_TIME {
				if let Err(error) = self.tick() {
					match error.downcast_ref::<SmartError>() {
						Some(SmartError::CarrierUnavailable) => {
							reload = true;
						}
						_ => Err(error).unwrap(),
					}
				}
				last_tick.add_assign(UPDATE_TIME);
			}

			let delta = (last_tick.elapsed().as_secs_f32() / UPDATE_TIME.as_secs_f32()).abs();
			if let Err(error) = self.draw(delta) {
				match error.downcast_ref::<SmartError>() {
					Some(SmartError::CarrierUnavailable) => {
						reload = true;
					}
					_ => Err(error).unwrap(),
				}
			}

			if reload {
				self.reload().unwrap();
				reload = false;
			}
		}
	}

	pub fn join_integrated(&mut self) -> Result<()> {
		let mut server = Server::new(&self.api, self.thread_pool.clone(), None)?;
		let player = Player::new("testing testing".to_string());
		let mut client_world = ClientWorld {
			networking: server.create_local_connection(PlayerJoinData {
				player: player.clone(),
			}),
			chunk: ChunkHandler::new(&self.rendering),
			player_entity_id: None,
			player,
			entity: EntityHandler::new(&self.rendering),
			integrated: Some(Box::new(server)),
		};

		// sync api
		client_world.reload(&self.api, &self.carrier);

		// join packet
		client_world
			.networking
			.send(ClientPacket::Player(ClientPlayerPacket::Join {}))?;
		self.world = Some(client_world);

		Ok(())
	}

	fn reload(&mut self) -> Result<()> {
		debug!(target: "reload@rustariac", "Reloading Client");
		rustaria::api::reload(&mut self.api, &mut self.carrier)?;

		let carrier = self.carrier.lock();
		let mut sprites = HashSet::new();
		prototypes!({
			for prototype in carrier.get_registry::<P>().iter() {
				prototype.get_sprites(&mut sprites);
			}
		});
		self.backend.instance_mut().supply_atlas(&self.api, sprites);

		if let Some(world) = &mut self.world {
			world.reload(&self.api, &self.carrier);
		}

		Ok(())
	}

	fn tick(&mut self) -> Result<()> {
		if let Some(world) = &mut self.world {
			world.tick(&mut self.camera, &mut self.control)?;
		}

		Ok(())
	}

	fn draw(&mut self, delta: f32) -> Result<()> {
		self.control.draw(&mut self.camera, delta);

		if let Some(world) = &mut self.world {
			world.draw(&mut self.camera, delta)?;
		}

		self.backend.instance_mut().backend.draw(&self.camera);
		Ok(())
	}
}
