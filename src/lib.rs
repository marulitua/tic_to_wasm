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

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    //alert(&format!("Hello, {}!", name));
    log(&format!("Hello, {}!", name));
}

// Strings can both be passed in and received
#[wasm_bindgen]
pub fn render() -> Bar {
    Bar::new(454)
}

#[wasm_bindgen]
pub struct Bar {
    contents: u32
}

#[wasm_bindgen]
impl Bar {
    pub fn from_str() {
        log(&format!("Bar->from_str"))
    }

    pub fn reset() {
        log(&format!("Bar->reset"))
    }

    pub fn new(contents: u32) -> Bar {
        Bar { contents }
    }
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

#[wasm_bindgen]
pub struct MyResult {
   data: Vec<u8>,
}

#[wasm_bindgen]
impl MyResult {
    pub fn add(&mut self, data: &[u8]) {
        self.data.push(4)
    }

    pub fn ptr(&self) -> *const u8 {
        &self.data[0]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// A struct will show up as a class on the JS side of things
#[wasm_bindgen]
pub struct Foo {
    contents: u32,
}

#[wasm_bindgen]
impl Foo {
    pub fn new() -> Foo {
        Foo { contents: 0 }
    }

    // Methods can be defined with `&mut self` or `&self`, and arguments you
    // can pass to a normal free function also all work in methods.
    pub fn add(&mut self, amt: u32) -> u32 {
        self.contents += amt;
        return self.contents
    }

    // You can also take a limited set of references to other types as well.
    pub fn add_other(&mut self, bar: &Bar) {
        self.contents += bar.contents;
    }

    // Ownership can work too!
    pub fn consume_other(&mut self, bar: Bar) {
        self.contents += bar.contents;
    }

    pub fn show(&self) {
        log_u32(self.contents)
    }
}
