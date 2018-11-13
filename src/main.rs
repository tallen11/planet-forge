extern crate png;
extern crate nalgebra;

mod export;
mod image;
mod renderer;

use export::{ Exporter };
use export::png_exporter::PNGExporter;
use image::pixel::ColorChannelData;
use image::image::Image;
use image::image_chunk::ImageChunk;

use renderer::renderer::Renderer;
use renderer::path_tracer_integrator::PathTracerIntegrator;

fn main() {
    let image_width = 300;
    let image_height = 200;

    let renderer = Renderer::new(Box::new(PathTracerIntegrator::new(10, 5)));

    let chunks = Image::chunkify(image_width, image_height, image_width, image_height);
    let image = Image::new(image_width, image_height, chunks);

    match PNGExporter::export_image_data_to_file(&image.get_raw_data(), image.get_width(), image.get_height(), "../out.png") {
        Ok(()) => println!("Export successful"),
        Err(error) => println!("Export failed: {}", error),
    }
}
