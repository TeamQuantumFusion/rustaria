use std::collections::HashSet;

use rustaria_api::ty::Tag;
use rustaria_api::ty::{Prototype, RawId};
use serde::Deserialize;

#[cfg(feature = "client")]
use crate::api::rendering::RenderingSystem;
use crate::entity::component::hitbox::HitboxComp;
use crate::entity::component::velocity::VelocityComp;

#[derive(Clone, Debug, Deserialize, frogelua::FromLua)]
pub struct EntityPrototype {
	pub velocity: Option<VelocityComp>,
	pub hitbox: Option<HitboxComp>,
	#[cfg(feature = "client")]
	pub rendering: Option<RenderingSystem>,
}

impl Prototype for EntityPrototype {
	type Item = ();

	fn create(&self, _: RawId) -> Self::Item {}

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
		"Entities"
	}
}
