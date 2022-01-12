use std::collections::HashMap;

use mlua::prelude::*;
use mooncake::mooncake;
use tracing::info;

use crate::{chunk::tile::TilePrototype, package};

package! {
    register, default
}

#[mooncake]
fn register(tiles: HashMap<String, TilePrototype>) -> LuaResult<()> {
    for (key, tile) in tiles {
        info!(?key, ?tile, "Registered tile");
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<TilePrototype> {
    lua.from_value(LuaValue::Table(t))
}
