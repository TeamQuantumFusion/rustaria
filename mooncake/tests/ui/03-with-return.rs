use mlua::prelude::*;
use mooncake::mooncake;

fn main() {
    let lua = Lua::new();
    let func = lua.create_function(fetch_me_a_veg).unwrap();
    lua.globals().set("fetch_me_a_veg", func).unwrap();
    lua.load(
        r#"
print(fetch_me_a_veg(2))
    "#,
    )
    .exec()
    .unwrap();
}

#[mooncake]
fn fetch_me_a_veg(idx: usize) -> LuaResult<Option<String>> {
    println!("full list of veg: {:?} | idx = {}", VEGGIES, idx);
    // careful - indexing starts at 1 in Lua ;)
    Ok(VEGGIES.get(idx - 1).map(|s| s.to_string()))
}

const VEGGIES: &[&str] = &["carrot", "lettuce", "okra"];
