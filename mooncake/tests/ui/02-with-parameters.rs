use mlua::prelude::*;
use mooncake::mooncake;
use std::collections::HashMap;

fn main() {
    let lua = Lua::new();
    let func = lua.create_function(register).unwrap();
    lua.globals().set("register", func).unwrap();
    lua.load(
        r#"
register({ hello = 3, answer = 42 }, "potat")
    "#,
    )
    .exec()
    .unwrap();
}

#[mooncake]
fn register(map: HashMap<String, u8>, id: String) -> LuaResult<()> {
    println!("got map with id \"{}\": {:?}", id, map);
    Ok(())
}
