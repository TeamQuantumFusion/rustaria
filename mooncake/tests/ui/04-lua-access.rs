use mlua::prelude::*;
use mooncake::mooncake;

fn main() {
    let lua = Lua::new();
    let func = lua.create_function(call_me).unwrap();
    lua.globals().set("call_me", func).unwrap();
    lua.load(
        r#"
function jumpin_around()
    print("Rust calling back to Lua! amazing")
end

print(call_me())
    "#,
    )
    .exec()
    .unwrap();
}

#[mooncake(lua)]
fn call_me() -> LuaResult<()> {
    lua.globals()
        .get::<_, LuaFunction>("jumpin_around")?
        .call(())
}
