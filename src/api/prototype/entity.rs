use legion::Entity;
use legion::storage::{ArchetypeSource, ArchetypeWriter, ComponentSource, EntityLayout};
use crate::world::entity::{EntityHandler, PositionComp, VelocityComp};
use serde::Deserialize;
use rustaria_api::lua_runtime::UserData;
use rustaria_api::prototype::Prototype;
use rustaria_api::RawId;

use rustaria_util::ty::pos::Pos;

#[derive(Clone, Debug, Deserialize)]
pub struct EntityPrototype {
	#[serde(default)]
	pub velocity: Option<VelocityCompPrototype>,
	#[cfg(feature = "client")]
	pub rendering: Option<crate::api::rendering::RenderingSystem>
}

impl UserData for EntityPrototype {}

impl Prototype for EntityPrototype {
	type Item = ();

	fn create(&self, _: RawId) -> Self::Item {}

	fn name() -> &'static str {
		"entity"
	}
}


#[derive(Clone, Debug, Deserialize, Default)]
pub struct VelocityCompPrototype {
	pub x: f32,
	pub y: f32,
}

impl Prototype for VelocityCompPrototype {
	type Item = VelocityComp;

	fn create(&self, _: RawId) -> Self::Item {
		VelocityComp {
			velocity: Pos {
				x: self.x,
				y: self.y
			}
		}
	}

	fn name() -> &'static str {
		"entity::velocity"
	}
}

impl UserData for VelocityCompPrototype {}
