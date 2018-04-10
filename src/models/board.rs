use models::tile::Tile;

pub struct Board {
    tiles: Vec<Tile>,
    size: u32
}

impl Board {
    pub fn new (size: u32) -> Board {
        let mut list_tiles = Vec::new();
        for i in 0..size {
            for j in 0..size {
                list_tiles.push(Tile::new())
            }
        }

        Board {
            tiles: list_tiles,
            size: size
        }
    }

    pub fn get_size(&self) -> usize {
        self.tiles.len()
    }
}
