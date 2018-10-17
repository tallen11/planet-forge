extern crate nalgebra;
extern crate png;
extern crate rand;

mod image;
mod png_writer;
mod ray;
mod camera;
mod intersectable;
mod material;
mod sphere;
mod renderer;
mod scene;

use nalgebra::Vector3;
use image::Image;
use png_writer::{ImageWriter, PNGWriter};
use sphere::Sphere;
use scene::Scene;
use renderer::Renderer;
use intersectable::Intersectable;

const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 200;
const SAMPLES_PER_PIXEL: usize = 100;
const BOUNCES_PER_RAY: usize = 1;

fn main() {
    let scene_objects: Vec<Box<Intersectable>> = vec![Box::new(Sphere::new(Vector3::new(0.0, 0.0, 1.0), 0.5))];
    let scene = Scene::new(scene_objects);

    let planet_renderer = Renderer::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, BOUNCES_PER_RAY);
    let image = planet_renderer.render(&scene);

    PNGWriter::write_image_to_file(image, "./output.png");
}

// macro_rules! v {
//     ($x:expr, $y:expr, $z:expr) => (nalgebra::Vector3::new($x as f64, $y as f64, $z as f64))
// }
