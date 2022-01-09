use std::collections::HashMap;

use mlua::prelude::*;
use tracing::info;

use crate::chunk::tile::Tile;


pub fn package(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    lua.create_table_from([
        ("register", lua.create_function(|_, v| register(v))?),
        ("default", lua.create_function(|l, v| default(l, v))?),
    ])
}
fn register(t: LuaTable<'_>) -> LuaResult<()> {
    for pair in t.pairs::<String, Tile>() {
        let (key, tile) = pair?;
        info!(?key, ?tile, "Registered tile");
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
fn default(lua: &Lua, t: LuaTable<'_>) -> LuaResult<Tile> {
    lua.from_value(LuaValue::Table(t))
}
