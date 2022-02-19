use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};
use crate::plugin_id;


#[mooncake::mooncake(lua)]
pub fn package() -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([
        ("trace", lua.create_function(trace)?),
        ("debug", lua.create_function(debug)?),
        ("info", lua.create_function(info)?),
        ("warn", lua.create_function(warn)?),
        ("error", lua.create_function(error)?),
    ])
}

#[mooncake(lua)]
fn trace(msg: String) -> LuaResult<()> {
    trace!("[{}]: {msg}", plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn debug(msg: String) -> LuaResult<()> {
    debug!("[{}]: {msg}", plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn info(msg: String) -> LuaResult<()> {
    info!("[{}]: {msg}", plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn warn(msg: String) -> LuaResult<()> {
    warn!("[{}]: {msg}", plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn error(msg: String) -> LuaResult<()> {
    error!("[{}]: {}", plugin_id(lua)?, msg);
    Ok(())
}
