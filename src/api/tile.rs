use std::collections::HashMap;
use mlua::Error;

use mlua::prelude::*;
use tokio::sync::mpsc::{UnboundedSender};

use mooncake::mooncake;

use crate::{chunk::tile::TilePrototype};
use crate::api::Prototype;
use crate::registry::Tag;


pub fn package(lua: &Lua, prototype_sender: UnboundedSender<Prototype>) -> LuaResult<LuaTable> {
    lua.create_table_from([
        ("register", lua.create_function(move |_, tiles| register(tiles, prototype_sender.clone()))?),
        ("default", lua.create_function(default)?)
    ])
}


fn register(tiles: HashMap<String, TilePrototype>, prototype_sender: UnboundedSender<Prototype>) -> LuaResult<()> {
    for (key, tile) in tiles {
        prototype_sender.send(Prototype::Tile(Tag::new(key), tile)).map_err(|err| Error::RuntimeError(err.to_string()))?;
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<TilePrototype> {
    lua.from_value(LuaValue::Table(t))
}
