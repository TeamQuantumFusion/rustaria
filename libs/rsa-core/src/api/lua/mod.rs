use anyways::ext::AuditExt;
use apollo::Lua;

use crate::err::Result;

mod log;

pub fn register(lua: &Lua) -> Result<()> {
	log::register(lua, &lua.globals()).wrap_err("Registering log")?;
	Ok(())
}
