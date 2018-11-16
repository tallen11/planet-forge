use image::pixel::{ColorChannelData, Pixel};
use image::image_chunk::{ImageChunkDescriptor, ImageChunk};

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

    pub fn chunkify(&self, chunk_width: u32, chunk_height: u32) -> Vec<ImageChunkDescriptor> {
        let mut chunks: Vec<ImageChunkDescriptor> = Vec::new();
        let rows = self.height / chunk_height + 1;
        let cols = self.width / chunk_width + 1;

        for row in 0..rows {
            for col in 0..cols {
                let chunk_row = row * chunk_height;
                let mut chunk_height_adjusted = chunk_height;
                if chunk_row + chunk_height_adjusted >= self.height {
                    chunk_height_adjusted = self.height - chunk_row;
                }

                let chunk_col = col * chunk_width;
                let mut chunk_width_adjusted = chunk_width;
                if chunk_col + chunk_width_adjusted >= self.width {
                    chunk_width_adjusted = self.width - chunk_col;
                }

                if chunk_width_adjusted > 0 && chunk_height_adjusted > 0 {
                    let descriptor = ImageChunkDescriptor::new(chunk_row, chunk_col, chunk_width_adjusted, chunk_height_adjusted);
                    chunks.push(descriptor);
                }
            }
        }

        chunks
    }

    pub fn set_chunk(&mut self, chunk: &ImageChunk) {
        for row in 0..chunk.get_height() {
            for col in 0..chunk.get_width() {
                let pixel = chunk.get_pixel(row, col);
                let abs_row = row + chunk.get_row();
                let abs_col = col + chunk.get_col();
                self.set_pixel(pixel, abs_row, abs_col);
            }
        }
    }

    fn set_pixel(&mut self, pixel: Pixel, row: u32, col: u32) {
        let index = (col + row * self.width) as usize;
        assert!(index < self.pixels.len());
        self.pixels[index] = pixel;
    }

    pub fn get_raw_data(&self) -> Vec<ColorChannelData> {
        self.pixels.iter()
                    .flat_map(|pixel| vec![pixel.red(), pixel.green(), pixel.blue()])
                    .collect()
    }
}
