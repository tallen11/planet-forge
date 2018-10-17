use rand::prelude::*;

use image::{Pixel, Image};
use camera::{Camera, BasicCamera};
use scene::Scene;

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    ray_bounce_count: usize,
}

impl Renderer {
    pub fn new(image_width: usize, image_height: usize, samples_per_pixel: usize, ray_bounce_count: usize) -> Renderer {
        Renderer {
            image_width: image_width,
            image_height: image_height,
            samples_per_pixel: samples_per_pixel,
            ray_bounce_count: ray_bounce_count,
        }
    }

    pub fn render(&self, scene: &Scene) -> Image {
        let mut rng = thread_rng();
        let camera = BasicCamera::new();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                for s in 0..self.samples_per_pixel {
                    // do rendering here...
                    // let mut color = 
                    for _ in 0..self.ray_bounce_count {
                        let p_x = (col as f64) - (self.image_width as f64 / 2.0) + rng.gen::<f64>();
                        let p_y = (row as f64) - (self.image_height as f64 / 2.0) + rng.gen::<f64>();
                        let ray = camera.generate_ray(p_x, p_y);
                        if let Some(result) = scene.find_intersection_in_scene(&ray) {
                            // calculate sometihng ....
                        } else {
                            // return background color...
                        }
                    }
                }
            }
        }

        Image::new(self.image_width, self.image_height)
    }
}
