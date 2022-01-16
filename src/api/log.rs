use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};

use crate::{package, api::get_plugin_id};

package! {
    trace, debug, info, warn, error
}

#[mooncake(lua)]
fn trace(msg: String) -> LuaResult<()> {
    trace!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
#[mooncake(lua)]
fn debug(msg: String) -> LuaResult<()> {
    debug!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
#[mooncake(lua)]
fn info(msg: String) -> LuaResult<()> {
    info!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
#[mooncake(lua)]
fn warn(msg: String) -> LuaResult<()> {
    warn!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
#[mooncake(lua)]
fn error(msg: String) -> LuaResult<()> {
    error!("[{}]: {}", get_plugin_id(lua)?, msg);
    Ok(())
}
