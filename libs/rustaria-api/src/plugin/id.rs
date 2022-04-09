use crate::lua::PluginContext;
use mlua::prelude::LuaResult;
use mooncake::mooncake;

pub type PluginId = String;

#[mooncake(lua)]
pub fn plugin_id() -> LuaResult<String> {
    let ctx = PluginContext::get(lua)?;
    Ok(ctx.id)
}
