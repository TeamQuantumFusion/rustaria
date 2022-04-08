mod log;
mod meta;

use mlua::Lua;
use mlua::prelude::{LuaResult, LuaUserData};
use crate::Plugin;

pub fn register_api(lua: &Lua) {

}


#[derive(Debug, Clone)]
pub struct PluginContext {
	pub id: String
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
			id: plugin.manifest.id.clone()
		}
	}
}

impl LuaUserData for PluginContext {}