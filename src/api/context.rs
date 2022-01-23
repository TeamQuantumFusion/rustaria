use mlua::prelude::*;

/// The context that is embedded in the Lua runtime, while running a plugin.
/// Implemented as a hidden userdata globally, this should _only_ be used
/// _internally_, and should not be relied upon by scripts.
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub plugin_id: String
}
impl PluginContext {
    pub fn get(lua: &Lua) -> LuaResult<Self> {
        lua.globals().get("_ctx")
    }
    pub fn set(self, lua: &Lua) -> LuaResult<()> {
        lua.globals().set("_ctx", self)
    }
}

impl LuaUserData for PluginContext {}