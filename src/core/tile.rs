
pub(crate) struct Tile{
    pixels: [u8; 64], // 16 bytes 8x8 tile, each pixel has two bit for color
}

impl Tile{
    pub fn new() -> Tile {
        let pixels: [u8; 64] = [0; 64];

        Tile {
            pixels
        }
    }
}