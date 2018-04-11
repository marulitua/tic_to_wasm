#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use models::{Board, Tile};
pub mod models;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    let w = Board::new(10, 1000, 1000);
    w.get_size() as i32
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(test)]
mod tests {
    use super::models::{Tile, Board};

    #[test]
    fn it_works() {
        let w = Board::new(3, 1000, 1000);
        assert_eq!(w.get_size(), 9);

        let w = Board::new(10, 1000, 1000);
        assert_eq!(w.get_size(), 100);
    }
}
