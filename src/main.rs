pub mod models;
use models::{Board, Tile};

fn main() {
    let w = Board::new(10, 1000, 1000);
    println!("there {} tiles", w.get_size());
    println!("this is the board {:?}", w);
}
