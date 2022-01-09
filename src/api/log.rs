use mlua::prelude::*;
use tracing::{debug, error, info, trace, warn};

pub fn package(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([
        ("info", lua.create_function(|_, v| info(v))?)
    ])
}
fn info(msg: String) -> LuaResult<()> {
    info!("{}", msg);
    Ok(())
}
