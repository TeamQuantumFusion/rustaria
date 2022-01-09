use mlua::prelude::*;

mod log;
mod tile;

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    globals.set("log", log::package(lua)?)?;
    globals.set("tile", tile::package(lua)?)?;
    Ok(())
}