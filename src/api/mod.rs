use mlua::prelude::*;
use tokio::sync::mpsc::{Sender, UnboundedSender};
use crate::chunk::tile::TilePrototype;
use crate::registry::Tag;

mod log;
#[macro_use]
pub(crate) mod macros;
mod tile;

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua, sender: UnboundedSender<Prototype>) -> LuaResult<()> {
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;

    preload.set("log", lua.create_function(log::package)?)?;
    preload.set("tile", lua.create_function(move |lua, _: ()| {
        tile::package(lua, sender.clone())
    })?)?;
    Ok(())
}

pub enum Prototype {
    Tile(String, TilePrototype)
}