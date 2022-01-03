use rustaria_api::it_adds_two;

#[no_mangle]
pub extern "C" fn setup() {
    println!("1 + 3 = {}", unsafe { it_adds_two(1, 3) });
}