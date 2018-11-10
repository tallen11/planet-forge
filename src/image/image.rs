use image::pixel::ColorChannelData;
use image::image_chunk::ImageChunk;

pub struct Image {
    width: u32,
    height: u32,
    chunks: Vec<ImageChunk>,
}

impl Image {
    pub fn new(width: u32, height: u32, chunks: Vec<ImageChunk>) -> Image {
        assert!(width > 0);
        assert!(height > 0);

        Image {
            width: width,
            height: height,
            chunks: chunks,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Converts the image into a ```Vec``` of ColorChannelData
    /// in the format R,G,B for writing to a file.
    ///
    /// Currently only supports single chunk for now...
    pub fn get_raw_data(&self) -> Vec<ColorChannelData> {
        let mut data: Vec<ColorChannelData> = Vec::new();
        
        let chunk = &self.chunks[0];
        for pixel in chunk.get_pixels() {
            data.push(pixel.get_red());
            data.push(pixel.get_green());
            data.push(pixel.get_blue());
        }

        data
    }

    /// Returns chunks of the specified size or smaller. Used to 
    /// allow parts of the image to be rendered in parallel if desired.
    ///
    /// Only returns single, full-sized chunk for now...
    pub fn chunkify(image_width: u32, image_height: u32, chunk_width: u32, chunk_height: u32) -> Vec<ImageChunk> {
        assert!(chunk_width > 0);
        assert!(chunk_height > 0);

        let mut chunks: Vec<ImageChunk> = Vec::new();

        let chunk = ImageChunk::new(0, 0, image_width, image_height);
        chunks.push(chunk);


        // let chunk_width = image.get_width() / chunk_count;
        // for i in 0..chunk_count-1 {
        //     let chunk = ImageChunk::new(0, i*chunk_width, chunk_width, image.get_height());
        //     chunks.push(chunk);
        // }

        // let last_chunk_col = (chunk_count - 1) * chunk_width;
        // let last_chunk = ImageChunk::new(0, last_chunk_col, image.get_width() - last_chunk_col, image.get_height());
        // chunks.push(last_chunk);

        chunks
    }
}
