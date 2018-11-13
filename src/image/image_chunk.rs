use image::pixel::{ ColorChannelData, Pixel };

pub struct ImageChunk {
    row: u32,
    col: u32,
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl ImageChunk {
    pub fn new(row: u32, col: u32, width: u32, height: u32) -> ImageChunk {
        ImageChunk {
            row: row,
            col: col,
            width: width,
            height: height,
            pixels: vec![Pixel::new(255,0,0); (width*height) as usize],
        }
    }

    pub fn set_pixel(&mut self, row: u32, col: u32, pixel: Pixel) {
        let index = (col + row * self.width) as usize;
        self.pixels[index] = pixel;
    }

    pub fn get_pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    pub fn get_pixel_indicies(&self) -> Vec<(u32, u32)> {
        let mut indicies: Vec<(u32, u32)> = vec![(0, 0); (self.width*self.height) as usize];
        for i in 0..self.width*self.height {
            indicies[i as usize] = (i / self.width + self.row, i % self.width + self.col);
        }

        indicies
    }
}
