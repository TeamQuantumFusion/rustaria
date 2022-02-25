use mlua::{prelude::*, Variadic};
use mooncake::mooncake;

use crate::registry::Tag;

use super::loader::PluginInput;

#[mooncake::mooncake(lua)]
pub fn package() -> LuaResult<LuaTable<'_>> {
    let make_id = lua.create_function(make_id)?;
    lua.create_table_from([
        ("plugin_id", lua.create_function(plugin_id)?),
        ("make_id", make_id.clone()),
        ("_", make_id),
    ])
}

#[mooncake(lua)]
fn plugin_id() -> LuaResult<String> {
    let ctx = PluginInput::get(lua)?;
    Ok(ctx.id)
}
#[mooncake(lua)]
fn make_id(strs: Variadic<String>) -> LuaResult<Tag> {
    match strs.len() {
        // just name
        1 => Ok(Tag {
            plugin_id: plugin_id(lua, ())?,
            name: strs[0].clone(),
        }),
        // both plugin id and name specified
        2 => Ok(Tag {
            plugin_id: strs[0].clone(),
            name: strs[1].clone(),
        }),
        _ => Err(LuaError::DeserializeError(
            "more than two strings provided".into(),
        )),
    }
}

