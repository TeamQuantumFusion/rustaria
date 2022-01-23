use mlua::prelude::*;
use mooncake::mooncake;

use super::context::PluginContext;

package! {
    plugin_id
}

#[mooncake(lua)]
fn plugin_id() -> LuaResult<String> {
    let ctx = PluginContext::get(lua)?;
    Ok(ctx.plugin_id)
}