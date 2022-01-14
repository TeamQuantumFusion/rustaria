use std::collections::HashMap;
use mlua::Error;

use mlua::prelude::*;
use tokio::sync::mpsc::{UnboundedSender};

use mooncake::mooncake;

use crate::{chunk::tile::TilePrototype};
use crate::api::PrototypeRequest;
use crate::registry::Tag;


pub fn package(lua: &Lua, prototype_sender: UnboundedSender<PrototypeRequest>) -> LuaResult<LuaTable> {
    lua.create_table_from([
        ("register", lua.create_function(move |lua, tiles| register(lua, tiles, prototype_sender.clone()))?),
        ("default", lua.create_function(default)?)
    ])
}


fn register(lua: &Lua, tiles: HashMap<String, TilePrototype>, prototype_sender: UnboundedSender<PrototypeRequest>) -> LuaResult<()> {
    for (key, tile) in tiles {
        let string = lua.globals().get("mod_id")?;
        let tag1 = Tag::new(string, key);
        prototype_sender.send(PrototypeRequest::Tile(tag1, tile)).map_err(|err| Error::RuntimeError(err.to_string()))?;
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<TilePrototype> {
    lua.from_value(LuaValue::Table(t))
}
