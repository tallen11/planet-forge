use rand::prelude::*;
use nalgebra::Vector3;

use image::{Pixel, PixelValue, Image};
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
        let mut image = Image::new(self.image_width, self.image_height);

        let mut rng = thread_rng();
        let camera = BasicCamera::new();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut avg_color = Vector3::new(0.0, 0.0, 0.0);
                for s in 0..self.samples_per_pixel {
                    // do rendering here...
                    let color = self.render_sample(row, col, &mut rng, &camera, &scene);
                    avg_color += color;
                }

                avg_color /= self.samples_per_pixel as f64;
                image.set_pixel(row, col, Pixel::new((avg_color.x * 255.0) as PixelValue, (avg_color.y * 255.0) as PixelValue, (avg_color.z * 255.0) as PixelValue));
            }
        }

        image
    }

    fn render_sample(&self, row: usize, col: usize, rng: &mut ThreadRng, camera: &Camera, scene: &Scene) -> Vector3<f64> {
        let mut color = Vector3::new(0.0_f64, 0.0, 0.0);
        for _ in 0..self.ray_bounce_count {
            let p_x = 2.0 * ((col as f64) - (self.image_width as f64 / 2.0) + rng.gen::<f64>()) / self.image_width as f64;
            let p_y = 2.0 * ((row as f64) - (self.image_height as f64 / 2.0) + rng.gen::<f64>()) / self.image_height as f64;
            let ray = camera.generate_ray(p_x, p_y);
            if let Some(result) = scene.find_intersection_in_scene(&ray) {
                // println!("{:?}", result.normal);
                color = 0.5 * Vector3::new(result.normal.x + 1.0, result.normal.y + 1.0, result.normal.z + 1.0);
            } else {
                color = Vector3::new(0.0_f64, 0.0, 0.0);
            }
        }

        color
    }
}
