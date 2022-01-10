use mlua::prelude::*;

mod log;
#[macro_use]
pub(crate) mod macros;
mod tile;

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<()> {
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;
    preload.set("log", lua.create_function(log::package)?)?;
    preload.set("tile", lua.create_function(tile::package)?)?;
    Ok(())
}
