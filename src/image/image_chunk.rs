use image::pixel::Pixel;

#[derive(Copy, Clone, Debug)]
pub struct ImageChunkDescriptor {
    row: u32,
    col: u32,
    width: u32,
    height: u32,
}

impl ImageChunkDescriptor {
    pub fn new(row: u32, col: u32, width: u32, height: u32) -> ImageChunkDescriptor {
        ImageChunkDescriptor {
            row: row,
            col: col,
            width: width,
            height: height,
        }
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_col(&self) -> u32 {
        self.col
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[derive(Clone)]
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
            pixels: vec![Pixel::black(); (width*height) as usize],
        }
    }

    pub fn from_descriptor(descriptor: ImageChunkDescriptor) -> ImageChunk {
        ImageChunk {
            row: descriptor.get_row(),
            col: descriptor.get_col(),
            width: descriptor.get_width(),
            height: descriptor.get_height(),
            pixels: vec![Pixel::black(); (descriptor.get_width()*descriptor.get_height()) as usize],
        }
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_col(&self) -> u32 {
        self.col
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, row: u32, col: u32) -> Pixel {
        let index = (col + row * self.width) as usize;
        assert!(index < self.pixels.len());
        return self.pixels[index];
    }

    pub fn set_pixel(&mut self, pixel: Pixel, row: u32, col: u32) {
        let index = (col + row * self.width) as usize;
        assert!(index < self.pixels.len());
        self.pixels[index] = pixel;
    }
}
