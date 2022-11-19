use log::error;
use pixels::{SurfaceTexture, Pixels, Error};
use winit::{window::{Window}};

#[derive(Copy, Clone)]
pub(crate) struct Tile{
    pub pixels: [u8; 64], // 16 bytes 8x8 tile, each pixel has two bit for color
}

pub(crate) struct Renderer{
    widthRes: usize,
    heightRes: usize,
    display: Vec<usize>,
    pixels: Pixels,
    vram: [u8; 8192],
    bgAddressingMode: bool, // true = addressing $8000; false = addressing $9000,
    tileBlock1: [Tile; 128],
    tileBlock2: [Tile; 128],
    tileBlock3: [Tile; 128]
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Renderer, Error> {
        let defaultWidth: usize = 160;
        let defaultHeight: usize = 144;

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(defaultWidth as u32, defaultHeight as u32, surface_texture)?
        };

        let bgAddressingMode = false;
        let vram: [u8; 8192] = [0; 8192];
        let tileBlock1: [Tile; 128] = [Tile{ pixels: [0; 64] } ; 128];
        let tileBlock2: [Tile; 128] = [Tile{ pixels: [0; 64] } ; 128];
        let tileBlock3: [Tile; 128] = [Tile{ pixels: [0; 64] } ; 128];

        Ok(Renderer {
            widthRes: defaultWidth,
            heightRes: defaultHeight,
            display: vec![0; defaultWidth * defaultHeight],
            pixels,
            vram,
            bgAddressingMode,
            tileBlock1,
            tileBlock2,
            tileBlock3
        })
    }

    pub fn setPixel(&mut self, mut x: usize, mut y: usize) -> bool {
        if x > self.widthRes {
            x -= self.widthRes;
        }

        if y > self.heightRes {
            y -= self.heightRes;
        }

        let pixelLoc: usize = x % self.widthRes + (y % self.heightRes * self.widthRes);

        self.display[pixelLoc] ^= 1;

        self.display[pixelLoc] == 0
    }

    pub fn clear(&mut self) {
        self.display = vec![0; self.widthRes * self.heightRes];
    }

    pub fn render(&mut self) -> bool {
        let frame = self.pixels.get_frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.display[i] == 0 {
                [0, 0, 0, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }

        self.pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
    }
}

