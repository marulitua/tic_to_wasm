//use models::{Board, Tile};
pub mod models;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return 14
}

#[cfg(test)]
mod tests {
    use super::models::{Tile};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let t = Tile::new();
        assert_eq!(t.get_size(), 10);
    }
}
