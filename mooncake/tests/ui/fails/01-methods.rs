//! Methods are unsupported, as of the time being
use mooncake::mooncake;

fn main() {
    // it won't compile anyway
}

struct A;

impl A {
    #[mooncake]
    fn it_works(&self) -> LuaResult<()> {
        println!("ey, it works!");
        Ok(())
    }
}
