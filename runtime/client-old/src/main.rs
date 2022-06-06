use std::collections::HashSet;
use std::ops::AddAssign;
use std::sync::Arc;
use std::time::{Duration, Instant};

use clap::Parser;
use glfw::{Action, ffi, Key, Modifiers, WindowEvent};
use rayon::{ThreadPool, ThreadPoolBuilder};

use rsa_core::api::{Api, Reloadable};
use rsa_core::error::Result;
use rsa_core::logging::{debug, LevelFilter};
use rsa_core::math::vec2;
use rsa_core::reload;
use rsa_core::settings::UPS;
use rsa_core::ty::{Prototype, Tag};
use rsa_network::client::ClientNetwork;
use rsac_backend::ClientBackend;
use rsac_backend::ty::Camera;
use rsac_glium_backend::GliumBackend;
use rustaria::entity::prototype::EntityPrototype;
use rustaria::chunk::layer::tile::TilePrototype;
use rustaria::packet::ClientPacket;
use rustaria::packet::entity::ClientEntityPacket;
use rustaria::packet::player::ClientPlayerPacket;
use rustaria::player::Player;
pub use rustaria::prototypes;
pub use rustaria::pt;
use rustaria::Server;
use rustaria::RichError;
use world::ClientWorld;

use crate::args::Args;
use crate::module::chunk::ChunkHandler;
use crate::module::controller::ControllerHandler;
use crate::module::entity::EntityHandler;
use crate::module::rendering::RenderingHandler;

mod args;
mod module;
mod world;

const DEBUG_MOD: Modifiers =
	Modifiers::from_bits_truncate(ffi::MOD_ALT + ffi::MOD_CONTROL + ffi::MOD_SHIFT);
const UPDATE_TIME: Duration = Duration::from_micros((1000000 / UPS) as u64);

fn main() -> Result<()> {
	let args = args::Args::parse();
	rsa_core::initialize(LevelFilter::Off)?;

	let mut client = Client::new(args)?;
	client.join_integrated()?;

	{
		let carrier = client.api.get_carrier();
		let prototype = carrier.get::<EntityPrototype>();
		let id = prototype.id_from_tag(&Tag::rsa("bunne")).expect("where bunne");
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

		let mut dir = std::env::current_dir()?;
		dir.push("plugins");
		let api = Api::new(dir, args.extra_plugin_paths)?;

		let mut client = Client {
			api,
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
					match error.downcast_ref::<RichError>() {
						Some(RichError::CarrierUnavailable) => {
							reload = true;
						}
						_ => Err(error).unwrap(),
					}
				}
				last_tick.add_assign(UPDATE_TIME);
			}

			let delta = (last_tick.elapsed().as_secs_f32() / UPDATE_TIME.as_secs_f32()).abs();
			if let Err(error) = self.draw(delta) {
				match error.downcast_ref::<RichError>() {
					Some(RichError::CarrierUnavailable) => {
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
		let mut server = Server::new(&self.api, self.thread_pool.clone())?;
		let player = Player::new("testing testing".to_string());

		let networking =
			ClientNetwork::new_integrated(server.network.integrated.as_mut().unwrap())?;
		let mut client_world = ClientWorld {
			networking,
			chunk: ChunkHandler::new(&self.rendering),
			player_entity_id: None,
			player,
			entity: EntityHandler::new(&self.rendering),
			integrated: Some(Box::new(server)),
		};

		// sync api
		client_world.reload(&self.api);

		// join packet
		client_world
			.networking
			.send(ClientPacket::Player(ClientPlayerPacket::Join {}))?;
		self.world = Some(client_world);

		Ok(())
	}

	fn reload(&mut self) -> Result<()> {
		debug!(target: "reload@rustariac", "Reloading Client");
		reload!((TilePrototype, EntityPrototype) => &mut self.api);

		let mut sprites = HashSet::new();
		let carrier = self.api.get_carrier();
		prototypes!({
			for prototype in carrier.get::<P>().iter() {
				prototype.get_sprites(&mut sprites);
			}
		});
		self.backend.instance_mut().supply_atlas(&self.api, sprites);

		if let Some(world) = &mut self.world {
			world.reload(&self.api);
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
		if let Some(world) = &mut self.world {
			world.draw(&mut self.camera, delta)?;
		}

		self.backend.instance_mut().backend.draw(&self.camera);
		Ok(())
	}
}
