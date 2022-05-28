use std::collections::HashMap;

use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::entity::world::EntityWorld;
use rustaria::SmartError::CarrierUnavailable;
use rsa_core::api::{Api, Reloadable};
use rsa_core::api::carrier::Carrier;
use rsa_core::error::{ContextCompat, Result};
use rsa_core::math::{vec2, Vector2D, WorldSpace};
use rsa_core::ty::Uuid;
use rsac_backend::builder::VertexBuilder;
use rsac_backend::ty::Camera;
use rsac_backend::{layer::LayerChannel, ty::PosTexture, ClientBackend};

use crate::BakedRenderingSystem;

pub struct WorldEntityDrawer {
	backend: ClientBackend,
	carrier: Option<Carrier>,
	entity_drawers: Vec<Option<BakedRenderingSystem>>,
	entities: HashMap<Uuid, Vector2D<f32, WorldSpace>>,

	layer: LayerChannel<PosTexture>,
}

impl WorldEntityDrawer {
	pub fn new(backend: &ClientBackend) -> WorldEntityDrawer {
		WorldEntityDrawer {
			carrier: None,
			backend: backend.clone(),
			entity_drawers: Vec::new(),
			entities: Default::default(),
			layer: backend.instance_mut().backend.new_layer_pos_tex(),
		}
	}

	pub fn draw(&mut self, camera: &Camera, world: &EntityWorld, delta: f32) -> Result<()> {
		self.carrier.as_ref().wrap_err(CarrierUnavailable)?;

		let mut builder = VertexBuilder::default();
		for (uuid, id) in &world.entities {
			let pos = self.get_entity_pos(uuid, world, delta);
			if let Some(Some(system)) = self.entity_drawers.get(id.index()) {
				system.push(&mut builder, camera, pos.x, pos.y);
			}
		}

		self.layer.supply(builder);

		Ok(())
	}

	pub fn get_entity_pos(
		&mut self,
		uuid: &Uuid,
		world: &EntityWorld,
		delta: f32,
	) -> Vector2D<f32, WorldSpace> {
		if let Some(pos) = world.position.get(uuid) {
			if !self.entities.contains_key(uuid) {
				self.entities.insert(*uuid, pos.position);
			}
			let old_pos = self.entities.get_mut(uuid).unwrap();
			let new_position = pos.position;
			old_pos.lerp(new_position, delta)
		} else {
			vec2(0.0, 0.0)
		}
	}

	pub fn tick(&mut self, camera: &Camera, world: &EntityWorld) -> Result<()> {
		self.carrier.as_ref().wrap_err(CarrierUnavailable)?;

		for (uuid, pos) in &world.position {
			if !self.entities.contains_key(uuid) {
				self.entities.insert(*uuid, pos.position);
			}
			*self.entities.get_mut(uuid).unwrap() = pos.position;
		}

		Ok(())
	}
}

impl Reloadable for WorldEntityDrawer {
	fn reload(&mut self, api: &Api) {
		let carrier = api.get_carrier();

		self.entity_drawers.clear();
		for prototype in carrier.get::<EntityPrototype>().iter() {
			self.entity_drawers.push(
				prototype
					.rendering
					.as_ref()
					.map(|system| BakedRenderingSystem::new(system, &self.backend)),
			);
		}

		self.carrier = Some(carrier);
		self.layer.mark_dirty();
	}
}
