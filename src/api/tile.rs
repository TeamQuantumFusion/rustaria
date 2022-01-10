use std::collections::HashMap;

use mlua::prelude::*;
use mooncake::mooncake;
use tracing::info;

use crate::{chunk::tile::Tile, package};

package! {
    register, default
}

#[mooncake]
fn register(tiles: HashMap<String, Tile>) -> LuaResult<()> {
    for (key, tile) in tiles {
        info!(?key, ?tile, "Registered tile");
    }
    Ok(())
}

// TODO: figure out how to have polymorphism here
#[mooncake(lua)]
fn default(t: LuaTable<'_>) -> LuaResult<Tile> {
    lua.from_value(LuaValue::Table(t))
}
