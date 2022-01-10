use mlua::prelude::*;
use mooncake::mooncake;
use tracing::{debug, error, info, trace, warn};

pub fn package(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([("info", lua.create_function(info)?)])
}

#[mooncake]
fn info(msg: String) -> LuaResult<()> {
    info!("{}", msg);
    Ok(())
}
