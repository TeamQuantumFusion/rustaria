use mlua::prelude::*;
use mooncake::mooncake;

use super::context::PluginContext;

pub fn package(lua: &Lua) -> LuaResult<LuaFunction> {
    lua.create_function(|lua, _: ()| {
        let make_id = lua.create_function(make_id)?;
        lua.create_table_from([
            ("plugin_id", lua.create_function(plugin_id)?),
            ("make_id", make_id.clone()),
            ("_", make_id),
        ])
    })
}

#[mooncake(lua)]
fn plugin_id() -> LuaResult<String> {
    let ctx = PluginContext::get(lua)?;
    Ok(ctx.plugin_id)
}
#[mooncake(lua)]
fn make_id(name: String) -> LuaResult<String> {
    let plugin_id = plugin_id(lua, ())?;
    Ok(format!("{plugin_id}:{name}"))
}
