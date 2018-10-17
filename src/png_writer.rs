use image::Image;

pub trait ImageWriter {
    fn write_image_to_file(image: Image, path_string: &str);
}

pub struct PNGWriter;

impl ImageWriter for PNGWriter {
    fn write_image_to_file(image: Image, path_string: &str) {
        use std::path::Path;
        use std::fs::File;
        use std::io::BufWriter;
        use png::{self, HasParameters};

        let path = Path::new(path_string);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, image.get_width() as u32, image.get_height() as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&image.export_to_raw()).unwrap();
    }
}
