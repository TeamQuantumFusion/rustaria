use std::collections::HashSet;
use std::ops::AddAssign;
use std::sync::Arc;
use std::time::{Duration, Instant};

use clap::Parser;
use eyre::{Report, Result};
use glfw::{ffi, Action, Key, Modifiers, WindowEvent};

use rayon::{ThreadPool, ThreadPoolBuilder};
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::network::packet::entity::{ClientEntityPacket, ServerEntityPacket};
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::SmartError;
use rustaria::{Server, UPS};
use rustaria_api::ty::{Prototype, Tag};
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_util::ty::pos::Pos;
use rustaria_util::{debug, info};
use rustariac_backend::ty::Camera;
use rustariac_backend::ClientBackend;
use rustariac_glium_backend::GliumBackend;

use crate::chunk::ChunkHandler;
use crate::controller::ControllerHandler;
use crate::entity::EntityHandler;

pub use rustaria::prototypes;
pub use rustaria::pt;

mod args;
mod chunk;
mod controller;
mod entity;

const DEBUG_MOD: Modifiers =
	Modifiers::from_bits_truncate(ffi::MOD_ALT + ffi::MOD_CONTROL + ffi::MOD_SHIFT);
const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS);

fn main() -> eyre::Result<()> {
	let args = args::Args::parse();
	rustaria_util::initialize()?;
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
			zoom: 30.0,
			screen_y_ratio: 0.0,
		},
		backend,
		thread_pool: Arc::new(ThreadPoolBuilder::new().num_threads(12).build().unwrap()),
	};

	client.reload()?;
	client.join_integrated()?;
	{
		let lock = client.carrier.lock();
		let prototype = lock.get_registry::<EntityPrototype>();
		let id = prototype
			.id_from_tag(&Tag::new("rustaria:bunne".to_string()).unwrap())
			.unwrap();
		let world = client.world.as_mut().unwrap();
		let pos = Pos { x: 5.0, y: 5.0 };
		world
			.entity
			.packet(ServerEntityPacket::New(id, pos))
			.unwrap();

		world
			.networking
			.send(ClientPacket::Entity(ClientEntityPacket::Spawn(id, pos)))
			.unwrap();
	}
	client.run();

	Ok(())
}

pub struct Client {
	thread_pool: Arc<ThreadPool>, // Api
	api: Api,
	carrier: Carrier,

	camera: Camera,
	control: ControllerHandler,
	world: Option<ClientWorld>,
	backend: ClientBackend,
}

impl Client {
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
						Some(err @ SmartError::CarrierUnavailable) => {
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
					Some(err @ SmartError::CarrierUnavailable) => {
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
		let mut client_world = ClientWorld {
			networking: server.create_local_connection(),
			chunk: ChunkHandler::new(&self.backend),
			entity: EntityHandler::new(&self.backend),
			integrated: Some(Box::new(server)),
		};

		// sync api
		client_world.reload(&self.api, &self.carrier);

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
			let lock = self.carrier.lock();
			let prototype = lock.get_registry::<EntityPrototype>();
			let id = prototype
				.id_from_tag(&Tag::new("rustaria:bunne".to_string()).unwrap())
				.unwrap();
			let pos = Pos { x: 5.0, y: 5.0 };
			world
				.entity
				.packet(ServerEntityPacket::New(id, pos))
				.unwrap();

			world
				.networking
				.send(ClientPacket::Entity(ClientEntityPacket::Spawn(id, pos)))
				.unwrap();
			world.tick(&self.camera)?;
		}

		Ok(())
	}

	fn draw(&mut self, delta: f32) -> Result<()> {
		self.control.tick(&mut self.camera, delta);

		if let Some(world) = &mut self.world {
			world.draw(&self.camera, delta)?;
		}

		self.backend.instance_mut().backend.draw(&self.camera);
		Ok(())
	}
}

pub type NetworkHandler =
	rustaria_network::networking::ClientNetworking<ServerPacket, ClientPacket>;

pub struct ClientWorld {
	networking: NetworkHandler,
	chunk: ChunkHandler,
	entity: EntityHandler,
	integrated: Option<Box<Server>>,
}

impl ClientWorld {
	pub fn tick(&mut self, camera: &Camera) -> Result<()> {
		self.chunk.tick(camera, &mut self.networking)?;
		self.entity.tick(camera, &self.chunk.chunks)?;
		if let Some(integrated) = &mut self.integrated {
			integrated.tick()?;
		}

		self.networking.poll::<Report, _>(|packet| match packet {
			ServerPacket::Chunk(packet) => self.chunk.packet(packet),
			ServerPacket::Entity(packet) => self.entity.packet(packet),
		})?;

		Ok(())
	}

	pub fn draw(&mut self, camera: &Camera, delta: f32) -> Result<()> {
		self.chunk.draw(camera);
		self.entity.draw(camera, delta)?;

		Ok(())
	}
}

impl Reloadable for ClientWorld {
	fn reload(&mut self, api: &Api, carrier: &Carrier) {
		self.chunk.reload(api, carrier);
		self.entity.reload(api, carrier);
		if let Some(server) = &mut self.integrated {
			server.reload(api, carrier);
		}
	}
}
