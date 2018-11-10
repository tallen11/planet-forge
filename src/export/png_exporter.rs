use export::{ Exporter };
use image::ColorChannelData;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

pub struct PNGExporter;

impl Exporter for PNGExporter {
    fn export_image_data_to_file(image_data: &Vec<ColorChannelData>, image_width: u32, image_height: u32, path_str: &str) -> std::io::Result<()> {
        let path = Path::new(path_str);
        let file = File::create(path)?;
        let ref mut writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, image_width, image_height);
        encoder.set(png::ColorType::RGB)
               .set(png::BitDepth::Eight);

        let mut png_writer = encoder.write_header()?;
        png_writer.write_image_data(&image_data[..])?;
        
        Ok(())
    }
}
