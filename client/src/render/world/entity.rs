use std::collections::HashSet;

use euclid::{Rect, Vector2D};
use glium::{uniform, Blend, DrawParameters, Program};
use rustaria::{
	api::{prototype::Prototype},
	ty::{identifier::Identifier, WS},
	world::entity::{
		component::{PhysicsComponent, PositionComponent, PrototypeComponent},
		EntityWorld,
	},
};

use apollo::{FromLua, Lua, Value, macros::*};
use rustaria::api::util::lua_table;
use anyways::Result;

use crate::{
	render::{
		atlas::Atlas,
		ty::{
			draw::Draw, mesh_buffer::MeshDrawer, mesh_builder::MeshBuilder, vertex::PosTexVertex,
		},
	},
	ClientApi, Frontend, PlayerSystem,
};

pub struct WorldEntityRenderer {
	drawer: MeshDrawer<PosTexVertex>,
}

impl WorldEntityRenderer {
	pub fn new(frontend: &Frontend) -> Result<WorldEntityRenderer> {
		Ok(WorldEntityRenderer {
			drawer: frontend.create_drawer()?,
		})
	}

	pub fn draw(
		&mut self,
		api: &ClientApi,
		player: &PlayerSystem,
		entity: &EntityWorld,
		program: &Program,
		draw: &mut Draw,
	) -> Result<()> {
		let mut builder = MeshBuilder::new();
		for (entity, (position, prototype, physics)) in entity
			.storage
			.query::<(&PositionComponent, &PrototypeComponent, &PhysicsComponent)>()
			.iter()
		{
			if let Some(renderer) = api.c_carrier.entity_renderer.get(prototype.id) {
				// If this entity is our player, we use its predicted position instead of its server confirmed position.
				let (mut position, mut vel) = (position.pos, physics.vel - physics.accel);
				if let Some(player_entity) = player.server_player {
					if player_entity == entity {
						if let Some(pos) = player.get_comp::<PositionComponent>() {
							position = pos.pos;
						}
						if let Some(physics) = player.get_comp::<PhysicsComponent>() {
							vel = physics.vel - physics.accel;
						}
					}
				}
				renderer.mesh(
					(position - vel).lerp(position, draw.timing.delta()),
					&mut builder,
				);
			}
		}
		self.drawer.upload(&builder)?;

		let uniforms = uniform! {
			screen_ratio: draw.frontend.aspect_ratio,
			atlas: &draw.atlas.texture,
			player_pos: draw.viewport.pos.to_array(),
			zoom: draw.viewport.zoom,
		};

		let draw_parameters = DrawParameters {
			blend: Blend::alpha_blending(),
			..DrawParameters::default()
		};

		self.drawer
			.draw(draw.frame, program, &uniforms, &draw_parameters)?;
		Ok(())
	}
}
pub struct EntityRenderer {
	pub image: Rect<f32, Atlas>,
	pub panel: Rect<f32, WS>,
}

impl EntityRenderer {
	pub fn mesh(&self, pos: Vector2D<f32, WS>, builder: &mut MeshBuilder<PosTexVertex>) {
		let mut rect = self.panel;
		rect.origin += pos;
		builder.push_quad((rect, self.image));
	}
}


#[derive(Debug)]
pub struct EntityRendererPrototype {
	pub image: Identifier,
	pub panel: Rect<f32, WS>,
}

impl EntityRendererPrototype {
	pub fn bake(&self, atlas: &Atlas) -> EntityRenderer {
		EntityRenderer {
			image: atlas.get(&self.image),
			panel: self.panel,
		}
	}

	pub fn get_sprites(&self, sprites: &mut HashSet<Identifier>) {
		sprites.insert(self.image.clone());
	}
}

impl Prototype for EntityRendererPrototype {
	type Output = EntityRenderer;

	fn get_name() -> &'static str { "entity_renderer" }
}

impl FromLua for EntityRendererPrototype {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		let table = lua_table(lua_value)?;
		Ok(EntityRendererPrototype {
			image: table.get("image")?,
			panel: table.get_ser("panel")?,
		})
	}
}
