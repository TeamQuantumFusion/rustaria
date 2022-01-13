use std::collections::HashMap;
use mlua::Error;

use mlua::prelude::*;
use tokio::sync::mpsc::{UnboundedSender};
use tracing::info;

use mooncake::mooncake;

use crate::{chunk::tile::TilePrototype};
use crate::api::Prototype;


pub fn package(lua: &Lua, sender: UnboundedSender<Prototype>) -> LuaResult<LuaTable> {
    lua.create_table_from([
        ("register", lua.create_function(move |_, tiles| register(tiles, sender.clone()))?),
        ("default", lua.create_function(default)?)
    ])
}


fn register(tiles: HashMap<String, TilePrototype>, sender: UnboundedSender<Prototype>) -> LuaResult<()> {
    for (key, tile) in tiles {
        info!(?key, ?tile, "Registered tile");
        let result = sender.send(Prototype::Tile(key, tile));
        if let Err(er) = result {
            return Err(Error::RuntimeError(er.to_string()));
        }
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<TilePrototype> {
    lua.from_value(LuaValue::Table(t))
}
