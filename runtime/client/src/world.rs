use crate::{vec2, InputModule, PlayerModule};
use rayon::ThreadPool;
use rsa_core::api::Api;
use rsa_core::error::{Result, WrapErr};
use rsa_network::client::ClientTickData;
use rsac_graphic::{Draw, GraphicSystem};
use rustaria::chunk::{Chunk, ChunkSystem};
use rustaria::entity::component::pos::PositionComp;
use rustaria::entity::packet::ServerEntityPacket;
use rustaria::entity::prototype::EntityPrototype;
use rustaria::entity::EntitySystem;
use rustaria::packet::{ClientPacket, ServerPacket};
use rustaria::world::World;
use rustaria::{ClientNetwork, Server};
use std::sync::Arc;
use rsa_core::logging::trace;
use rsa_core::ty::{ChunkPos, Tag};
use rsac_graphic::camera::Camera;
use rsac_graphic::render::WorldRenderer;
use rustaria::chunk::layer::ChunkLayer;
use rustaria::chunk::layer::tile::TilePrototype;
use rustaria::packet::player::ClientPlayerPacket;

pub struct ClientWorld {
	api: Api,
	integrated_server: Option<Box<Server>>,

	// our view of the world
	world: World,
	network: ClientNetwork,
	player: PlayerModule,
	tick: u32,

	renderer: WorldRenderer,
}

impl ClientWorld {
	pub fn new_integrated(api: &Api, graphics: &mut GraphicSystem, thread_pool: Arc<ThreadPool>) -> Result<ClientWorld> {
		let mut server = Server::new_integrated(api, thread_pool)?;
		server.reload(api);

		let mut renderer = WorldRenderer::new(graphics)?;
		renderer.reload(api, graphics)?;

		let mut world = World::new();
		let carrier = api.get_carrier();
		let result = carrier.get::<TilePrototype>().create_from_tag(&Tag::rsa("dirt"))?;
		let pos = ChunkPos {
			x: 0,
			y: 0,
		};

		let pos2 = ChunkPos {
			x: 0,
			y: 1,
		};

		let chunk = Chunk {
			tiles: ChunkLayer::new_copy(result)
		};


		server.world.chunks.put_chunk(pos, chunk.clone());
		server.world.chunks.put_chunk(pos2, chunk.clone());
		world.chunks.put_chunk(pos, chunk.clone());
		world.chunks.put_chunk(pos2, chunk);
		renderer.notify_chunk(pos);
		renderer.notify_chunk(pos2);

		let network = ClientNetwork::new_integrated(server.network.integrated.as_mut().unwrap())?;
		network.send(ClientPacket::Player(ClientPlayerPacket::Join()))?;

		Ok(ClientWorld {
			api: api.clone(),
			network,
			integrated_server: Some(Box::new(server)),
			world: world,
			player: PlayerModule::new(api),
			tick: 0,
			renderer
		})
	}

	pub fn setup_camera(&mut self, camera: &mut Camera, tick_pos: f32) -> Result<()>{
		self.player.setup_camera(camera, &self.world, tick_pos)
	}

	pub fn tick(&mut self, input: &mut InputModule) -> Result<()> {
		self.tick += 1;

		self.world.tick()?;
		input.apply_movement(&mut self.player)?;
		self.player.tick(self.tick, &self.network)?;

		if let Some(integrated) = &mut self.integrated_server {
			integrated.tick()?;
			match self.network.tick()? {
				ClientTickData::Received(data) => {
					for packet in data {
						self.packet(packet).wrap_err("Packet fail")?;
					}
				}
				ClientTickData::Disconnected => {
					todo!("Disconnecting")
				}
			}
		}
		Ok(())
	}

	pub fn draw(&mut self, draw: &mut Draw) -> Result<()> {
		self.renderer.draw(draw, &self.world)?;
		Ok(())
	}

	pub fn reload(&mut self, api: &Api, graphics: &mut GraphicSystem) -> Result<()> {
		if let Some(server) = &mut self.integrated_server {
			server.reload(api);
		}
		self.renderer.reload(api, graphics)?;
		Ok(())
	}

	fn packet(&mut self, packet: ServerPacket) -> Result<()> {
		match packet {
			ServerPacket::Chunk(_) => {}
			ServerPacket::Entity(packet) => match packet {
				ServerEntityPacket::Pos(tick, entity, pos) => {
					if let Ok(mut pos_comp) = self.world.entities.get_mut::<PositionComp>(entity) {
						pos_comp.position = pos;
					}
					self.player.check_position(tick, entity, &self.world)?;
				}
				ServerEntityPacket::Spawn(tick, entity, id) => {
					self.world.entities.spawn_at(
						entity,
						vec2(0.0, 0.0),
						id,
						self.api
							.get_carrier()
							.get::<EntityPrototype>()
							.prototype_from_id(id),
					);
				}
			},
			ServerPacket::Player(packet) => {
				self.player.packet(packet, &mut self.world)?;
			}
		}
		Ok(())
	}
}
