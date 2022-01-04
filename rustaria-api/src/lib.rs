macro_rules! api_func {
    ($($NAME:tt($($P_NAME:ident : $P_TYPE: ty),*) -> $RETURN:ty;)*) => {
        $(
        pub fn $NAME($($P_NAME: $P_TYPE),*) -> $RETURN {
            unsafe {
                extern "C" {
                    fn $NAME($($P_NAME: $P_TYPE),*) -> $RETURN;
                }
                $NAME($($P_NAME),*)
            }
        }
        )*
    };
}

api_func! {
    it_adds_two(a: i32, b: i32) -> i32;
}