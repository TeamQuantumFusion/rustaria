use crate::vec2;
use rsa_core::api::Api;
use rsa_core::error::{ContextCompat, Result};
use rsa_core::logging::{trace, warn};
use rsa_core::math::{Vector2D, WorldSpace};
use rsac_graphic::camera::Camera;
use rustaria::entity::component::humanoid::HumanoidComp;
use rustaria::entity::component::pos::PositionComp;
use rustaria::entity::packet::ClientEntityPacket;
use rustaria::entity::{Entity, EntitySystem};
use rustaria::packet::player::ServerPlayerPacket;
use rustaria::packet::ClientPacket;
use rustaria::world::World;
use rustaria::ClientNetwork;
use std::collections::VecDeque;
use rustaria::chunk::ChunkSystem;

type PlayerDir = Vector2D<f32, WorldSpace>;

pub struct PlayerModule {
	api: Api,
	// Our player
	player_entity: Option<Entity>,

	// Prediction stuff
	unprocessed_commands: VecDeque<(u32, PlayerDir)>,
	commands: Vec<PlayerDir>,
	//
	base_system: EntitySystem,
	prediction_system: EntitySystem,
	input_dir: PlayerDir,
	//
	send_dir: PlayerDir,
	old_pos: f32,
}

impl PlayerModule {
	pub fn new(api: &Api) -> Self {
		Self {
			api: api.clone(),
			player_entity: None,
			unprocessed_commands: Default::default(),
			commands: vec![],
			base_system: EntitySystem::new(),
			prediction_system: EntitySystem::new(),
			input_dir: Default::default(),
			send_dir: Default::default(),
			old_pos: 0.0,
		}
	}

	pub fn setup_camera(
		&mut self,
		camera: &mut Camera,
		world: &World,
		tick_pos: f32,
	) -> Result<()> {
		if tick_pos < self.old_pos {
			self.old_pos = -(1.0 - self.old_pos) ;
		}
		let delta = tick_pos - self.old_pos;

		//self.prediction_system.tick(&world.chunks, delta)?;

		// Change the speed that we are actually going to send to the server
		self.send_dir += self.input_dir * delta;
		self.old_pos = tick_pos;

		// Set position to our predicted player_entity
		if let Some(player_entity) = self.player_entity {
			self.correct_offset(player_entity, world)?;

			if let Ok(pos) = world.entities.get::<PositionComp>(player_entity) {
				camera.pos = pos.position;
			} else {
				warn!("no entity")
			}
		}
		Ok(())
	}

	pub fn tick(&mut self, tick: u32, network: &ClientNetwork) -> Result<()> {
		// Send our speed at this tick
		network.send(ClientPacket::Entity(ClientEntityPacket::PlayerDirection(
			tick,
			self.send_dir,
		)))?;
		self.unprocessed_commands.push_front((tick, self.input_dir));
		self.send_dir = vec2(0.0, 0.0);
		Ok(())
	}

	pub fn check_position(
		&mut self,
		tick: u32,
		entity: Entity,
		world: &World,
	) -> Result<()> {
		if let Some(player_entity) = &self.player_entity {
			if entity == *player_entity {
				// Remove all events that the server has now applied.
				while let Some((value_tick, speed)) = self.unprocessed_commands.pop_back() {
					// Move the base server entity forward.
					// This totally ignores if the server sends a different speed, this is intentional.
					// By this being on the predicted speed we can safely isolate the error amount by doing
					// self.server_entity - self.base_server_entity, this lets us correct it in a sneaky timeframe.
					if let Ok(mut comp) = self.base_system.get_mut::<HumanoidComp>(*player_entity) {
						comp.dir = speed;
					}
					self.base_system.tick(&world.chunks, 1.0)?;

					// If we reach the tick that we currently received,
					// stop as the next events are the ones that the server has not yet seen.
					if value_tick == tick {
						break;
					}
				}

				self.compile_prediction(*player_entity, &world.chunks)?;
			}
		}

		Ok(())
	}

	pub fn packet(&mut self, packet: ServerPlayerPacket, world: &mut World) -> Result<()> {
		match packet {
			ServerPlayerPacket::Attach { entity } => {
				let mut builder = world
					.entities
					.clone(entity)
					.wrap_err("Player does not exist in the world")?;

				// Insert it into our prediction systems
				self.base_system.clear();
				self.base_system.insert(entity, builder.build());
				self.prediction_system.clear();
				self.prediction_system.insert(entity, builder.build());

				self.player_entity = Some(entity);
			}
		}
		Ok(())
	}

	pub fn set_movement_direction(&mut self, movement_direction: Vector2D<f32, WorldSpace>) {
		self.input_dir = movement_direction;
	}

	// If the server says a different value try to correct it without freaking the player out.
	fn correct_offset(&mut self, entity: Entity, world: &World) -> Result<()> {
		let server_entity = world.entities.get::<PositionComp>(entity)?;
		let mut base_server_entity = self.base_system.get_mut::<PositionComp>(entity)?;
		let mut prediction_entity = self.prediction_system.get_mut::<PositionComp>(entity)?;

		let server_offset = server_entity.position - base_server_entity.position;
		let distance = server_offset.abs();

		// If the distance is too big just teleport the donut.
		let length = distance.length();
		if length > 0.1 {
			base_server_entity.position = server_entity.position;
			prediction_entity.position = server_entity.position;
		} else if length > 0.0 {
			// Slightly drift the donut.
			let mut x_amount = distance.x.clamp(0.0, 0.0005);
			if server_offset.x < 0.0 {
				x_amount = -x_amount;
			}

			let mut y_amount = distance.y.clamp(0.0, 0.0005);
			if server_offset.y < 0.0 {
				y_amount = -y_amount;
			}

			let amount = vec2(x_amount, y_amount);
			base_server_entity.position += amount;
			prediction_entity.position += amount;
		}
		Ok(())
	}

	// When a client receives a packet, rebase the base_server_entity and
	// then apply the events not yet to be responded by the server.
	fn compile_prediction(&mut self, entity: Entity, chunks: &ChunkSystem) -> Result<()> {
		// Put prediction on the server value
		let mut builder = self
			.base_system
			.clone(entity)
			.wrap_err("Could not find player entity")?;
		self.prediction_system.insert(entity, builder.build());

		// If reconciliation is on, we apply values that the server has not yet processed.
		for (_, speed) in &self.unprocessed_commands {
			self.prediction_system.get_mut::<HumanoidComp>(entity)?.dir = *speed;
			self.prediction_system.tick(chunks, 1.0)?;
		}

		self.prediction_system.get_mut::<HumanoidComp>(entity)?.dir = self.input_dir;
		Ok(())
	}
}
