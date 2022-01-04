use wasmer::{Exports, Store};

fn _it_adds_two(a: i32, b: i32) -> i32 {
    a + b
}

pub fn dump_exports(store: &Store) -> Exports {
    exports! {
        store;
        _it_adds_two
    }
}

macro_rules! exports {
    ($store:expr; $($func:ident),+) => {
        use wasmer::{Exports, Extern, Function};
        [$((
            stringify!($func).to_string(),
            Extern::from(Function::new_native($store, $func)),
        )),+]
        .into_iter()
        .collect::<Exports>()
    };
}
use exports;
