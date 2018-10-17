use rand::prelude::*;
use nalgebra::Vector3;

use image::{Pixel, PixelValue, Image};
use camera::{Camera, BasicCamera};
use scene::Scene;
use intersectable::IntersectableType;
use ray::Ray;

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

    // TODO: Refactor this to return an ImageSlice and combine all slices together later to
    // allow for parallelizing the render process...
    pub fn render(&self, scene: &Scene) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        // TODO: Change to xoroshiro rng perhaps for greater speed although
        // I don't know what the rand crate is using under the hood...
        let mut rng = thread_rng();
        let camera = BasicCamera::new();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut avg_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    // TODO: This can be simplified...
                    let p_x = 2.0 * ((col as f64) - (self.image_width as f64 / 2.0) + rng.gen::<f64>()) / self.image_width as f64;
                    let p_y = 2.0 * ((row as f64) - (self.image_height as f64 / 2.0) + rng.gen::<f64>()) / self.image_height as f64;
                    let color = self.render_sample(p_y, p_x, &camera, &scene, self.ray_bounce_count);
                    avg_color += color;
                }

                avg_color /= self.samples_per_pixel as f64;
                image.set_pixel(row, col, Pixel::new((avg_color.x * 255.0) as PixelValue, (avg_color.y * 255.0) as PixelValue, (avg_color.z * 255.0) as PixelValue));
            }
        }

        image
    }

    fn render_sample(&self, x: f64, y: f64, camera: &Camera, scene: &Scene, max_bounces: usize) -> Vector3<f64> {
        let mut color = Vector3::new(0.0, 0.0, 0.0);
        let mut ray = camera.generate_ray(x, y);
        // let mut coefficient = 1.0_f64;
        for _ in 0..max_bounces {
            if let Some(result) = scene.find_intersection_in_scene(&ray) {
                match result.i_type {
                    IntersectableType::Light(emission) => {
                        let emitted_light = emission * (ray.get_direction().dot(&result.normal)).abs();
                        color *= emitted_light;
                        break;
                    }

                    IntersectableType::Solid => {
                        color = Vector3::new(1.0, 0.0, 0.0);
                        // let new_ray_direction = Vector3::new();
                        // ray = Ray::new(result.intersection_point, );
                    }
                }
                // color = 0.5 * Vector3::new(result.normal.x + 1.0, result.normal.y + 1.0, result.normal.z + 1.0);
            } else {
                break;
            }
        }

        color
    }
}
