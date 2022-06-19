use hecs::{BuiltEntityClone, EntityBuilderClone};
use tracing::{error_span, info};

use crate::{
	api::{luna::table::LunaTable, prototype::Prototype},
	ty::id::Id,
	world::entity::component::{
		CollisionComponent, GravityComponent, HumanoidComponent, PhysicsComponent,
		PositionComponent, PrototypeComponent,
	},
};
use apollo::impl_macro::*;

pub struct EntityDesc {
	pub template: BuiltEntityClone,
}

#[lua_impl]
impl EntityDesc {}

#[derive(Debug)]
pub struct EntityPrototype {
	pub position: PositionComponent,
	pub velocity: Option<PhysicsComponent>,
	pub collision: Option<CollisionComponent>,
	pub humanoid: Option<HumanoidComponent>,
	pub gravity: Option<GravityComponent>,
}

impl EntityPrototype {
	pub fn bake(self, id: Id<Self>) -> EntityDesc {
		info!("{self:?}");
		let mut builder = EntityBuilderClone::new();
		builder.add(self.position.clone());
		builder.add(PrototypeComponent { id: id.build() });
		if let Some(comp) = self.velocity.as_ref() {
			builder.add(comp.clone());
		};
		if let Some(comp) = self.collision.as_ref() {
			builder.add(comp.clone());
		};
		if let Some(comp) = self.humanoid.as_ref() {
			builder.add(comp.clone());
		};
		if let Some(comp) = self.gravity.as_ref() {
			builder.add(comp.clone());
		};
		EntityDesc {
			template: builder.build(),
		}
	}
}

impl Prototype for EntityPrototype {
	type Output = EntityDesc;

	fn get_name() -> &'static str { "entity" }

	fn from_lua(table: LunaTable) -> eyre::Result<Self> {
		let _span = error_span!(target: "lua", "entity").entered();
		Ok(EntityPrototype {
			position: table.get_ser("position")?,
			velocity: table.get_ser("velocity")?,
			collision: table.get("collision")?,
			humanoid: table.get_ser("humanoid")?,
			gravity: table.get_ser("gravity")?,
		})
	}
}
