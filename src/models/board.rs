use models::tile::Tile;

pub struct Board {
    tiles: Vec<Tile>,
    size: u32
}

impl Board {
    pub fn new (size: u32) -> Board {
        Board {
            tiles: vec![],
            size: size
        }
    }
}
