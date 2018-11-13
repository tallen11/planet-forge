use image::pixel::{ColorChannelData, Pixel};

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        assert!(width > 0);
        assert!(height > 0);

        Image {
            width: width,
            height: height,
            pixels: vec![Pixel::black(); (width*height*3) as usize],
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_pixel(&mut self, pixel: Pixel, row: u32, col: u32) {
        let index = (col + row * self.width) as usize;
        self.pixels[index] = pixel;
    }

    pub fn get_raw_data(&self) -> Vec<ColorChannelData> {
        self.pixels.iter()
                    .flat_map(|pixel| vec![pixel.red(), pixel.green(), pixel.blue()])
                    .collect()
    }
}
