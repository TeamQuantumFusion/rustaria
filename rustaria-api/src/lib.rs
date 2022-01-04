extern "C" {
    fn _it_adds_two(a: i32, b: i32) -> i32;
}

pub fn it_adds_two(a: i32, b: i32) -> i32 {
    unsafe { _it_adds_two(a, b) }
}
