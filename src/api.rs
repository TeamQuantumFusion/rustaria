use rsa_core::api::{Api};
use rsa_core::api::carrier::Carrier;
use rsa_core::error::Result;

use crate::api::prototype::entity::EntityPrototype;
use crate::api::prototype::tile::TilePrototype;

#[macro_use]
pub mod prototype;

#[cfg(feature = "client")]
pub mod rendering;
pub mod ty;

// Register everything
pub fn reload(api: &mut Api) -> Result<()> {
	let mut reload = api.reload();
	reload.register::<TilePrototype>()?;
	reload.register::<EntityPrototype>()?;
	reload.reload()?;
	reload.collect::<TilePrototype>()?;
	reload.collect::<EntityPrototype>()?;
	reload.apply();
	Ok(())
}
