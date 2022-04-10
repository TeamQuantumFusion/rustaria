use mlua::prelude::*;

use mooncake::mooncake;
use rustaria_util::{debug, error, info, trace, warn};

use crate::plugin::id::plugin_id;

#[mooncake(lua)]
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
    trace!(target: &plugin_id(lua, ())?, "{msg}");
    Ok(())
}
#[mooncake(lua)]
fn debug(msg: String) -> LuaResult<()> {
    debug!(target: &plugin_id(lua, ())?, "{msg}");
    Ok(())
}
#[mooncake(lua)]
fn info(msg: String) -> LuaResult<()> {
    info!(target: &plugin_id(lua, ())?, "{msg}");
    Ok(())
}
#[mooncake(lua)]
fn warn(msg: String) -> LuaResult<()> {
    warn!(target: &plugin_id(lua, ())?, "{msg}");
    Ok(())
}
#[mooncake(lua)]
fn error(msg: String) -> LuaResult<()> {
    error!(target: &plugin_id(lua, ())?, "{msg}");
    Ok(())
}
