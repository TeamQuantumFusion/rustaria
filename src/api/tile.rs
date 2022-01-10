use std::collections::HashMap;

use mlua::prelude::*;
use mooncake::mooncake;
use tracing::info;

use crate::chunk::tile::Tile;

pub fn package(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([
        ("register", lua.create_function(register)?),
        ("default", lua.create_function(default)?),
    ])
}

#[mooncake]
fn register(t: HashMap<String, Tile>) -> LuaResult<()> {
    for (key, tile) in t {
        info!(?key, ?tile, "Registered tile");
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<Tile> {
    lua.from_value(LuaValue::Table(t))
}
