#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
mod board;

use wasm_bindgen::prelude::*;
use board::Board;

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

//wasm calculate the next move
//only accept string of board
#[wasm_bindgen]
pub fn next_move(board: &str) -> u32 {
   log(&format!("Hello, {}!", board));
   guess(board);
   56
}

fn guess(board: &str) -> u32 {
   let tiles = convert_board(board);

   let board = Board {
      tiles: tiles
   };
   log(&board.log());
   log(&format!("peek the board => {:?}", board));
   board.get_size()
}

fn convert_board(string_board: &str) -> Vec<u32> {
   let numbers: Vec<u32> =
      string_board.split(',')
      .map(|s| s.parse().unwrap())
      .collect();

   numbers
   }

#[cfg(test)]
mod tests {
   use super::{guess, convert_board};

   #[test]
   fn it_can_guess() {
      // 1 0 0
      // 0 0 0
      // 0 0 0
      assert_eq!(guess("0,0,0,0,0,0,0,0"), 2);

      // 1-1 0
      // 1 0 0
      // 0 0 0
      assert_eq!(guess("1,-1,0,1,0,0,0,0"), 7);

      // 1 0-1
      // 1 1 0
      //-1 0 0
      assert_eq!(guess("1,0,-1,1,0,-1,0,0"), 6);

      // 1 0 0
      // 1 1 0
      //-1 0-1
      assert_eq!(guess("1,0,0,1,1,0,0,0"), 6);

      // 1-1 0
      // 1 1-1
      //-1 0 0
      assert_eq!(guess("1,-1,0,1,1,-1,-1,0,0"), 9);
   }

   #[test]
   fn it_can_convert_board() {
      let result = convert_board("0,0,0,1,0,1");

      assert_eq!(result, [0, 0, 0, 1, 0, 1]);
   }
}
