use eyre::Result;

use crate::api::prototype::entity::EntityPrototype;
use rustaria_api::registry::{Registry, RegistryBuilder};
use rustaria_api::ty::Tag;
use rustaria_api::{Api, Carrier};
use rustaria_util::ty::pos::Pos;
use rustaria_util::ty::Rectangle;

use crate::api::prototype::tile::TilePrototype;
use crate::api::rendering::{Pane, RenderingSystem};
use crate::entity::hitbox::HitboxComp;
use crate::entity::velocity::VelocityComp;

#[macro_use]
pub mod prototype;

#[cfg(feature = "client")]
pub mod rendering;
pub mod ty;

macro_rules! register {
    ($($TAG:literal: $PROTOTYPE:expr),*) => {
	     {
		     let mut builder = RegistryBuilder::new();
		     $(
		     builder.register(
			     Tag::new($TAG)?,
			     $PROTOTYPE,
		     );
		     )*
	        builder
	     }
    };
}
// Register everything
pub fn reload(api: &mut Api, carrier: &mut Carrier) -> Result<()> {
	let mut reload = api.reload(carrier);

	reload.add_registry(register!(
		"rustaria:air": TilePrototype {
			collision: false,
			sprite: None,
			connection: Default::default(),
		},
		"rustaria:dirt": TilePrototype {
			collision: true,
			sprite: Some(Tag::new("rustaria:lab.png")?),
			connection: Default::default(),
		}
	))?;

	let bunne = EntityPrototype {
		velocity: Some(VelocityComp {
			velocity: Pos { x: 2.0, y: 0.0 },
		}),
		hitbox: Some(HitboxComp {
			hitbox: Rectangle {
				x: 0.0,
				y: 0.0,
				width: 1.0,
				height: 0.5,
			},
		}),
		rendering: Some(RenderingSystem::Static(Pane {
			x_offset: 0.0,
			y_offset: 0.0,
			width: 1.0,
			height: 0.5,
			sprite: Tag::new("rustaria:glisco.png")?,
		})),
	};
	reload.add_registry(register!("rustaria:bunne": bunne))?;

	reload.reload();
	Ok(())
}
