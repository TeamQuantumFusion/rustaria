use eyre::{ContextCompat, Result};
use rustaria::api::prototype::entity::EntityPrototype;
use rustaria::entity::world::EntityWorld;
use rustaria::SmartError::CarrierUnavailable;
use rustaria_api::{Api, Carrier, Reloadable};
use rustaria_util::math::{Vector2D, WorldSpace};
use rustaria_util::ty::Pos;
use rustaria_util::Uuid;
use rustariac_backend::builder::VertexBuilder;
use rustariac_backend::ty::Camera;
use rustariac_backend::{layer::LayerChannel, ty::PosTexture, ClientBackend};
use std::collections::HashMap;

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
			if let Some(pos) = world.position.get(uuid) {
				if !self.entities.contains_key(uuid) {
					self.entities.insert(*uuid, pos.position);
				}
				let old_pos = self.entities.get_mut(uuid).unwrap();
				let pos = pos.position.lerp(*old_pos, delta);

				if let Some(Some(system)) = self.entity_drawers.get(id.index()) {
					system.push(&mut builder, camera, pos.x, pos.y);
				}
			}
		}

		self.layer.supply(builder);

		Ok(())
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
	fn reload(&mut self, _: &Api, carrier: &Carrier) {
		self.carrier = Some(carrier.clone());
		let instance = carrier.lock();
		let registry = instance.get_registry::<EntityPrototype>();

		self.entity_drawers.clear();
		for prototype in registry.iter() {
			self.entity_drawers.push(
				prototype
					.rendering
					.as_ref()
					.map(|system| BakedRenderingSystem::new(system, &self.backend)),
			);
		}

		self.layer.mark_dirty();
	}
}
