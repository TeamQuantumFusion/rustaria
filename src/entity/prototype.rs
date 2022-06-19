use hecs::EntityBuilder;
use rsa_core::ty::{Prototype, RawId, Tag};
use std::collections::HashSet;
use crate::api::rendering::RenderingSystem;
use crate::entity::component::gravity::GravityComp;
use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::humanoid::HumanoidComp;
use crate::entity::component::prototype::PrototypeComp;
use crate::entity::component::physics::PhysicsComp;

// please when adding new shit add also to mod.rs
#[derive(Clone, Debug, serde::Deserialize, frogelua::FromLua)]
pub struct EntityPrototype {
	pub velocity: Option<PhysicsComp>,
	pub hitbox: Option<HitboxComp>,
	pub gravity: Option<GravityComp>,
	pub humanoid: Option<HumanoidComp>,
	#[cfg(feature = "client")]
	pub rendering: Option<RenderingSystem>,
}

impl Prototype for EntityPrototype {
	type Item = EntityBuilder;

	fn create(&self, id: RawId) -> Self::Item {
		let mut builder = EntityBuilder::new();
		builder.add(PrototypeComp(id));

		if let Some(comp) = &self.hitbox {
			builder.add(comp.clone());
		}

		if let Some(comp) = &self.velocity {
			builder.add(comp.clone());
		}
		if let Some(comp) = &self.gravity {
			builder.add(comp.clone());
		}

		if let Some(comp) = &self.humanoid {
			builder.add(comp.clone());
		}

		builder
	}

	fn get_sprites(&self, sprites: &mut HashSet<Tag>) {
		#[cfg(feature = "client")]
		if let Some(system) = &self.rendering {
			match system {
				RenderingSystem::Static(pane) => {
					sprites.insert(pane.sprite.clone());
				}
				RenderingSystem::State(states) => {
					for pane in states.values() {
						sprites.insert(pane.sprite.clone());
					}
				}
			}
		}
	}

	fn lua_registry_name() -> &'static str {
		"entity"
	}
}
