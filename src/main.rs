extern crate png;
extern crate threadpool;

mod util;
mod export;
mod image;
mod renderer;
mod term;

use std::sync::Arc;

use export::{ Exporter };
use export::png_exporter::PNGExporter;
use image::pixel::ColorChannelData;
use image::image::Image;

use renderer::primitive::point::Point;
use renderer::primitive::vec3::Vec3;
use renderer::scene::scene::Scene;
use renderer::camera::pinhole_camera::PinholeCamera;
use renderer::renderer::Renderer;
use renderer::integrator::IntegratorType;
use renderer::path_tracer_integrator::PathTracerIntegrator;
use renderer::object::sphere::Sphere;
use renderer::object::light::Light;
use renderer::material::material::LambertianMaterial;

fn main() {
    let image_width = 400;
    let image_height = 200;
    let samples_per_pixel = 500;
    let max_bounces_per_ray = 50;

    let camera = PinholeCamera::new();
    
    let mut scene = Scene::new();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, -0.1, 1.0), 0.2, Box::new(LambertianMaterial::new(Vec3::new(0.0, 0.3, 0.8))))));
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, -100.3, 1.0), 100.0, Box::new(LambertianMaterial::new(Vec3::new(1.0, 1.0, 1.0))))));
    // scene.add_object(Box::new(Light::new(Point::new(0.5, 0.0, 1.0), 0.2)));
    scene.add_object(Box::new(Light::new(Point::new(-2.5, 0.0, 1.0), 1.0)));
    // scene.add_object(Box::new(Light::new(Point::new(15.0, 0.0, -20.0), 13.0)));

    let camera = PinholeCamera::new();

    let renderer = Renderer::new(image_width,
                                 image_height,
                                 samples_per_pixel,
                                 &camera,
                                 &scene,
                                 IntegratorType::PathTracer(max_bounces_per_ray),
                                 8);
    // let renderer = Renderer::new(image_width, image_height, samples_per_pixel, Box::new(PathTracerIntegrator::new(max_bounces_per_ray)), 8);
    
    let image = renderer.render();

    match PNGExporter::export_image_data_to_file(&image.get_raw_data(), image.get_width(), image.get_height(), "./out.png") {
        Ok(()) => println!("Export successful"),
        Err(error) => println!("Export failed: {}", error),
    }
}
