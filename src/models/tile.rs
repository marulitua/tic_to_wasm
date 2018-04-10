#[derive(Debug)]
pub struct Tile {
    colour: String,
    pub width: u32,
    pub height: u32,
    top: u32,
    left: u32,
    text: String,
    textColour: String
}

impl Tile {
    pub fn new(colour: String, width: u32, height: u32, top: u32, left: u32, text: String, textColour: String) -> Tile {
        Tile {
            colour,
            width,
            height,
            top,
            left,
            text,
            textColour,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.width
    }
}
