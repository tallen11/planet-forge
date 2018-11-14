extern crate png;
extern crate nalgebra;

mod util;
mod export;
mod image;
mod renderer;

use export::{ Exporter };
use export::png_exporter::PNGExporter;
use image::pixel::ColorChannelData;
use image::image::Image;

use renderer::primitive::point::Point;
use renderer::scene::scene::Scene;
use renderer::camera::pinhole_camera::PinholeCamera;
use renderer::renderer::Renderer;
use renderer::path_tracer_integrator::PathTracerIntegrator;
use renderer::object::sphere::Sphere;
use renderer::object::light::Light;

fn main() {
    let image_width = 300;
    let image_height = 200;

    let camera = PinholeCamera::new();
    
    let mut scene = Scene::new();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 0.0, 1.0), 0.5)));
    scene.add_object(Box::new(Light::new(Point::new(-1.0, 1.0, -1.0), 1.0)));

    let renderer = Renderer::new(image_width, image_height, 10, Box::new(PathTracerIntegrator::new(5)));
    
    let image = renderer.render(&camera, &scene);

    // let image = Image::new(image_width, image_height);

    match PNGExporter::export_image_data_to_file(&image.get_raw_data(), image.get_width(), image.get_height(), "./out.png") {
        Ok(()) => println!("Export successful"),
        Err(error) => println!("Export failed: {}", error),
    }
}
