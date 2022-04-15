
use crate::{plugin::PluginContext, ty::Tag};

mod log;
pub mod reload;

use mlua::{prelude::{LuaTable, LuaResult}, Lua, ExternalResult};
use rustaria_util::info;

pub fn register_preload(lua: &Lua) -> LuaResult<()>{
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;
    preload.set("log", lua.create_function(log::register)?)?;
    lua.globals().set("Tag", lua.create_function( |lua, value: String| {
        info!("Sthit");
        Tag::new_lua(value, lua).to_lua_err()
    })?)?;
    Ok(())  
}

pub fn ctx(lua: &Lua) -> PluginContext {
    // SHOULD NEVER FAIL!!! CAN ONLY HAPPEN IF:
    // - a plugin removed ctx (wtf) 
    // - lua got initialized somewhere else (wtf)
    lua.globals().get("ctx").expect("Context is missing.")
}
