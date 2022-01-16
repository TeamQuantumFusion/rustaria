use mlua::prelude::*;

pub struct Meta {
    pub mod_id: String,
}
impl Meta {
    pub fn into_module(self, lua: &Lua) -> LuaResult<LuaFunction> {
        lua.create_function(move |lua, _: ()| lua.create_table_from([
            ("plugin_id", self.mod_id.clone()),
        ]))
    }
}

