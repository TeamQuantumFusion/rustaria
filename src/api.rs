use eyre::Result;

use crate::api::prototype::entity::EntityPrototype;
use rustaria_api::registry::{Registry, RegistryBuilder};
use rustaria_api::ty::Tag;
use rustaria_api::{Api, Carrier};

use crate::api::prototype::tile::TilePrototype;
use crate::api::rendering::{Pane, RenderingSystem};

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
			sprite: None,
			connection: Default::default(),
		},
		"rustaria:dirt": TilePrototype {
			sprite: Some(Tag::new("rustaria:lab.png")?),
			connection: Default::default(),
		}
	))?;

	reload.add_registry(register!(
		"rustaria:bunne": EntityPrototype {
			velocity: None,
			rendering: Some(RenderingSystem::Static(Pane {
				x_offset: 0.0,
				y_offset: 0.0,
				width: 10.0,
				height: 10.0,
				sprite: Tag::new("rustaria:glisco.png")?,
			}))
		}
	))?;

	reload.reload();
	Ok(())
}
