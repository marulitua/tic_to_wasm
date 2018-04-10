#[derive(Debug)]
pub struct Tile {
    width: u32,
    height: u32,
    top: u32,
    left: u32,
    text: String
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            width: 10,
            height: 10,
            top: 5,
            left: 5,
            text: String::from("Yesss")
        }
    }

    pub fn get_size(&self) -> u32 {
        self.width
    }
}
