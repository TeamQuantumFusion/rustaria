use mlua::prelude::*;
use tokio::sync::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use crate::chunk::tile::TilePrototype;
use crate::chunk::wall::WallPrototype;
use crate::registry::Tag;

mod log;
#[macro_use]
pub(crate) mod macros;
mod tile;

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<UnboundedReceiver<Prototype>> {
    let (send, rec) = tokio::sync::mpsc::unbounded_channel();
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;

    preload.set("log", lua.create_function(log::package)?)?;
    preload.set("tile", lua.create_function(move |lua, _: ()| {
        tile::package(lua, send.clone())
    })?)?;
    Ok(rec)
}

pub enum Prototype {
    Tile(Tag, TilePrototype),
    Wall(Tag, WallPrototype),
}