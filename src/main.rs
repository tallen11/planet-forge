extern crate png;
extern crate nalgebra;

mod export;
mod image;

use export::{ Exporter };
use export::png_exporter::PNGExporter;
use image::ColorChannelData;

fn main() {

    let data: Vec<ColorChannelData> = Vec::new();

    match PNGExporter::export_image_data_to_file(&data, 10, 10, "./output.png") {
        Ok(()) => println!("Export successful"),
        Err(error) => println!("Export failed: {}", error),
    }
}
