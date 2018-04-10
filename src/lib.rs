//use models::{Board, Tile};
pub mod models;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return 2020
}

#[cfg(test)]
mod tests {
    use super::models::{Tile, Board};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let t = Tile::new();
        assert_eq!(t.get_size(), 10);

        let w = Board::new(3);
        assert_eq!(w.get_size(), 9);

        let w = Board::new(10);
        assert_eq!(w.get_size(), 100);
    }
}
