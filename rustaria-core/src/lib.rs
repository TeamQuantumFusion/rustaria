use rustaria_api::it_adds_two;

#[no_mangle]
pub extern "C" fn initialize() {
    println!("69 + 420 = {}", it_adds_two(69, 420));
}
