use models::tile::Tile;

pub struct Board {
    tiles: Vec<Tile>,
    size: u32,
    maxHeight: u32,
    maxWidth: u32
}

impl Board {
    pub fn new (size: u32, maxWidth: u32, maxHeight: u32) -> Board {
        let mut list_tiles = Vec::new();

        // Set the value content (x,y) axis
        let mut x = 20;
        let mut y;
        let width = 80;
        let height = 80;

        for _i in 1..size+1 {
            y = 20;
            for j in 1..size+1 {
                list_tiles.push(Tile::new(
                          String::from("#05EFFF"),
                          width,
                          height,
                          y,
                          x,
                          String::from(""),
                          String::from("#fff")
                        ));

                // get the y axis for next content
                y = y + list_tiles[(j-1) as usize].height + 5;
                if y >= maxHeight - list_tiles[(j-1) as usize].height {
                    break;
                }
            }
            //get the x axis for next content
            x = x + list_tiles[0].width + 5;
            if x >= maxWidth - list_tiles[0].width {
                break;
            }
        }

        Board {
            tiles: list_tiles,
            size: size,
            maxHeight: maxHeight,
            maxWidth: maxWidth
        }
    }

    pub fn get_size(&self) -> usize {
        self.tiles.len()
    }
}
