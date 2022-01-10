use mlua::prelude::*;
use mooncake::mooncake;

fn main() {
    let lua = Lua::new();
    let func = lua.create_function(it_works).unwrap();
    lua.globals().set("it_works", func).unwrap();
    lua.load(r#"it_works()"#).exec().unwrap();
}

#[mooncake]
fn it_works() -> LuaResult<()> {
    println!("ey, it works!");
    Ok(())
}
