use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
extern "C" {
    pub fn it_adds_two(a: i32, b: i32) -> i32;
}