#[macro_export]
macro_rules!
package {
    ($($func:expr),*) => {
        pub fn package(lua: &Lua) -> LuaResult<LuaFunction> {
            lua.create_function(|lua, _: ()| lua.create_table_from([
                $( (stringify!($func), lua.create_function($func)?) ),*
            ]))
        }
    };
}
