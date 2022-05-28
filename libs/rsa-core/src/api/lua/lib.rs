use mlua::Lua;

mod log;

pub fn load_builtin(lua: &Lua) -> mlua::Result<()> {
	let globals = lua.globals();
	log::register(lua, &globals)?;
	Ok(())
}