use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};

use crate::package;

package! {
    trace, debug, info, warn, error
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
