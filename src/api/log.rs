use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};

use crate::{api::get_plugin_id, package};

package! {
    trace, debug, info, warn, error
}

#[mooncake(lua)]
fn trace(msg: String) -> LuaResult<()> {
    trace!("[{}]: {msg}", get_plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn debug(msg: String) -> LuaResult<()> {
    debug!("[{}]: {msg}", get_plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn info(msg: String) -> LuaResult<()> {
    info!("[{}]: {msg}", get_plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn warn(msg: String) -> LuaResult<()> {
    warn!("[{}]: {msg}", get_plugin_id(lua)?);
    Ok(())
}
#[mooncake(lua)]
fn error(msg: String) -> LuaResult<()> {
    error!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
