use mlua::{prelude::*, Variadic};
use mooncake::mooncake;

use crate::plugin::id::plugin_id;
use crate::tag::Tag;

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
fn make_id(strs: Variadic<String>) -> LuaResult<Tag> {
    let id = plugin_id(lua, ())?;
    let s = match strs.len() {
        // just name
        1 => format!("{}:{}", id, strs[0]),
        // both plugin id and name specified
        2 => format!("{}:{}", strs[0], strs[1]),
        _ => {
            return Err(LuaError::DeserializeError(
                "more than two strings provided".into(),
            ))
        }
    };
    let tag = Tag::from_string(s)
        .expect("Constructed tag is not colon-delimited - this is certainly a bug!");
    Ok(tag)
}
