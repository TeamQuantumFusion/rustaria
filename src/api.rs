use rustaria_api::{Api, Carrier};
use rustaria_util::error::Result;

use crate::api::prototype::entity::EntityPrototype;
use crate::api::prototype::tile::TilePrototype;

#[macro_use]
pub mod prototype;

#[cfg(feature = "client")]
pub mod rendering;
pub mod ty;

// Register everything
pub fn reload(api: &mut Api, carrier: &mut Carrier) -> Result<()> {
	let mut reload = api.reload(carrier);
	reload.register::<TilePrototype>()?;
	reload.register::<EntityPrototype>()?;
	reload.reload()?;
	reload.collect::<TilePrototype>()?;
	reload.collect::<EntityPrototype>()?;
	reload.apply();
	Ok(())
}
