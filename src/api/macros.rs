#[macro_export]
macro_rules!
package {
    ($($func:expr),*) => {
        #[mooncake::mooncake(lua)]
        pub fn package() -> LuaResult<LuaTable<'_>> {
            lua.create_table_from([
                $( (stringify!($func), lua.create_function($func)?) ),*
            ])
        }
    };
}
