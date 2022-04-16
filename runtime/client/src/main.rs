use std::collections::HashSet;
use std::ops::AddAssign;
use std::time::{Duration, Instant};

use eyre::{Report, Result};
use glfw::{ffi, Action, Key, Modifiers, WindowEvent};

use rustaria::network::packet::{ClientPacket, ServerPacket};
pub use rustaria::prototypes;
pub use rustaria::pt;
use rustaria::SmartError;
use rustaria::{Server, UPS};
use rustaria_api::ty::Prototype;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_util::{debug, info};
use rustariac_backend::ty::Viewport;
use rustariac_backend::ClientBackend;
use rustariac_glium_backend::GliumBackend;

use crate::chunk::ChunkHandler;
use crate::controller::ControllerHandler;
use crate::entity::EntityHandler;

mod chunk;
mod controller;
mod entity;

const DEBUG_MOD: Modifiers =
	Modifiers::from_bits_truncate(ffi::MOD_ALT + ffi::MOD_CONTROL + ffi::MOD_SHIFT);
const UPDATE_TIME: Duration = Duration::from_micros(1000000 / UPS as u64);

fn main() -> eyre::Result<()> {
	rustaria_util::initialize()?;
	let backend = ClientBackend::new(GliumBackend::new)?;

	let carrier = Carrier::default();
	let mut dir = std::env::current_dir()?;
	dir.push("plugins");
	let api = Api::new(dir, vec!["../../../plugin".into()])?;

	let mut client = Client {
		api,
		carrier,
		world: None,
		control: ControllerHandler::new(),
		view: Viewport {
			position: [0.0, 0.0],
			zoom: 30.0,
		},
		backend,
	};

	client.reload()?;
	client.join_integrated()?;
	client.run();

	Ok(())
}

pub struct Client {
	// Api
	api: Api,
	carrier: Carrier,

	view: Viewport,
	control: ControllerHandler,
	world: Option<ClientWorld>,
	backend: ClientBackend,
}

impl Client {
	pub fn run(&mut self) {
		let mut last_tick = Instant::now();
		let mut last_delta = 0f32;

		let mut reload = false;
		while !self.backend.instance().backend.window().should_close() {
			for event in self.backend.instance_mut().backend.poll_events() {
				match event {
					WindowEvent::Scroll(_, y) => {
						self.view.zoom += y as f32;
					}
					WindowEvent::Key(Key::R, _, Action::Release, DEBUG_MOD) => {
						reload = true;
					}
					_ => {}
				}

				self.control.consume_event(event);
			}

			while last_tick.elapsed() >= UPDATE_TIME {
				if let Err(error) = self.tick() {
					match error.downcast_ref::<SmartError>() {
						Some(err @ SmartError::CarrierUnavailable) => {
							info!("{}", err);
							reload = true;
						}
						_ => Err(error).unwrap(),
					}
				}
				last_tick.add_assign(UPDATE_TIME);
			}

			if reload {
				self.reload().unwrap();
				reload = false;
			}

			let delta = ((last_tick.elapsed().as_secs_f32() / UPDATE_TIME.as_secs_f32())
				- last_delta)
				.abs();
			self.draw(delta);
			last_delta = delta;
		}
	}

	pub fn join_integrated(&mut self) -> Result<()> {
		let mut server = Server::new(12, None)?;
		let mut client_world = ClientWorld {
			networking: server.create_local_connection(),
			chunk: ChunkHandler::new(&self.backend),
			entity: EntityHandler {},
			integrated: Some(Box::new(server)),
		};

		// sync api
		client_world.reload(&self.api, &self.carrier);

		self.world = Some(client_world);
		Ok(())
	}

	fn reload(&mut self) -> Result<()> {
		debug!("Reloading Client");
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
			world.tick(&self.view)?;
		}

		Ok(())
	}

	fn draw(&mut self, delta: f32) {
		self.control.tick(&mut self.view, delta);

		if let Some(world) = &mut self.world {
			world.draw(&self.view);
		}

		self.backend.instance_mut().backend.draw(&self.view);
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
	pub fn tick(&mut self, view: &Viewport) -> Result<()> {
		self.chunk.tick(view, &mut self.networking)?;
		if let Some(integrated) = &mut self.integrated {
			integrated.tick()?;
		}

		self.networking.poll::<Report, _>(|packet| match packet {
			ServerPacket::Chunk(packet) => self.chunk.packet(packet),
			ServerPacket::Entity(packet) => self.entity.packet(packet),
		})?;

		Ok(())
	}

	pub fn draw(&mut self, view: &Viewport) {
		self.chunk.draw(view);
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
