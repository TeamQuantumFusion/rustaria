use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};

pub fn package(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([
        ("trace", lua.create_function(trace)?),
        ("debug", lua.create_function(debug)?),
        ("info", lua.create_function(info)?),
        ("warn", lua.create_function(warn)?),
        ("error", lua.create_function(error)?),
    ])
}

#[mooncake]
fn trace(msg: String) -> LuaResult<()> {
    trace!("{}", msg);
    Ok(())
}
#[mooncake]
fn debug(msg: String) -> LuaResult<()> {
    debug!("{}", msg);
    Ok(())
}
#[mooncake]
fn info(msg: String) -> LuaResult<()> {
    info!("{}", msg);
    Ok(())
}
#[mooncake]
fn warn(msg: String) -> LuaResult<()> {
    warn!("{}", msg);
    Ok(())
}
#[mooncake]
fn error(msg: String) -> LuaResult<()> {
    error!("{}", msg);
    Ok(())
}
