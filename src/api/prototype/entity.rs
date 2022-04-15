use mlua::UserData;
use rustaria_api::{ty::{Prototype, RawId}};
use serde::{Deserialize, Serialize};

use rustaria_util::ty::pos::Pos;

use crate::{world::entity::VelocityComp, api::rendering::RenderingSystem};

#[derive(Clone, Debug,  Serialize, Deserialize)]
pub struct EntityPrototype {
	#[serde(default)]
	pub velocity: Option<VelocityCompPrototype>,
	#[cfg(feature = "client")]
	pub rendering: Option<RenderingSystem>
}


impl Prototype for EntityPrototype {
	type Item = ();

	fn create(&self, _: RawId) -> Self::Item {}

	fn lua_registry_name() -> &'static str {
		"Entities"
	}
}



#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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

	fn lua_registry_name() -> &'static str {
		"entity::velocity"
	}
}

impl UserData for VelocityCompPrototype {

}