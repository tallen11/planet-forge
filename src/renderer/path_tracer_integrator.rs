use renderer::integrator::{Integrator, Radiance};
use renderer::scene::scene::Scene;
use renderer::ray::Ray;

pub struct PathTracerIntegrator {
    max_ray_bounces: u32,
}

impl PathTracerIntegrator {
    pub fn new(max_ray_bounces: u32) -> PathTracerIntegrator {
        PathTracerIntegrator {
            max_ray_bounces: max_ray_bounces,
        }
    }
}

impl Integrator for PathTracerIntegrator {
    fn compute_radiance(&self, ray: Ray, scene: &Scene) -> Radiance {
        if let Some(result) = scene.find_intersection(ray) {

        }

        Radiance::new(0.0, 0.0, 0.0)
    }
}

/*

Algorithm:
    1. Check for intersections with surfaces:
        if Light:
            

        Else:



*/
