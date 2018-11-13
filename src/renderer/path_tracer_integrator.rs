use renderer::integrator::{Integrator, Radiosity};
use image::image_chunk::ImageChunk;
use renderer::scene::scene::Scene;

use renderer::ray::Ray;

pub struct PathTracerIntegrator {
    rays_per_pixel: u32,
    max_ray_bounces: u32,
}

impl PathTracerIntegrator {
    pub fn new(rays_per_pixel: u32, max_ray_bounces: u32) -> PathTracerIntegrator {
        PathTracerIntegrator {
            rays_per_pixel: rays_per_pixel,
            max_ray_bounces: max_ray_bounces,
        }
    }
}

impl Integrator for PathTracerIntegrator {
    fn compute_radiosity(&self, screen_location_x: f32, screen_location_y: f32) -> Radiosity {
        Radiosity::new(0.0, 0.0, 0.0)
    }
}
