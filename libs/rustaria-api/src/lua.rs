mod log;
mod meta;

use crate::{info, Plugin};

use mlua::prelude::{LuaResult, LuaTable, LuaUserData};
use mlua::{Function, Lua};
use rustaria_util::Result;
pub fn register_api(lua: &Lua) -> Result<()> {
    info!("Registering api");
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;
    preload.set("log", lua.create_function(log::package)?)?;
    preload.set("meta", lua.create_function(meta::package)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct PluginContext {
    pub id: String,
}

impl PluginContext {
    pub fn get(lua: &Lua) -> LuaResult<Self> {
        lua.globals().get("_ctx")
    }
    pub fn set(self, lua: &Lua) -> LuaResult<()> {
        lua.globals().set("_ctx", self)
    }
}

impl From<&Plugin> for PluginContext {
    fn from(plugin: &Plugin) -> Self {
        PluginContext {
            id: plugin.manifest.id.clone(),
        }
    }
}

impl LuaUserData for PluginContext {}


pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new() -> LuaRuntime {
        LuaRuntime {
            lua: Lua::new()
        }
    }

}