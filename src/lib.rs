use models::{Board, Tile};
pub mod models;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    let w = Board::new(10, 1000, 1000);
    w.get_size() as i32
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
