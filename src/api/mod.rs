use mlua::prelude::*;

mod log;
mod tile;

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<()> {
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;
    preload.set("log", lua.create_function(|lua, _: ()| log::package(lua))?)?;
    preload.set(
        "tile",
        lua.create_function(|lua, _: ()| tile::package(lua))?,
    )?;
    Ok(())
}
