//! Holds our luna interface
use anyways::{ext::AuditExt, Result};
use apollo::Lua;

mod log;
pub mod registry_builder;
pub mod reload;
pub mod stargate;

pub fn register(lua: &Lua) -> Result<()> {
	log::register(lua, &lua.globals()).wrap_err("Registering log")?;
	Ok(())
}
