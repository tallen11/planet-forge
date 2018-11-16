use renderer::integrator::{Integrator, Radiance};
use renderer::scene::scene::Scene;
use renderer::ray::Ray;
use renderer::object::ObjectType;
use renderer::material::material::Material;
use util::xoroshiro_rng::XoroshiroRNG;
use renderer::primitive::vec3::Vec3;

pub struct PathTracerIntegrator {
    max_ray_bounces: u32,
}

impl PathTracerIntegrator {
    pub fn new(max_ray_bounces: u32) -> PathTracerIntegrator {
        PathTracerIntegrator {
            max_ray_bounces: max_ray_bounces,
        }
    }

    fn compute_radiance_vec(&self, ray: Ray, scene: &Scene, depth: u32, rng: &mut XoroshiroRNG) -> Vec3 {
        if depth >= self.max_ray_bounces {
            return Vec3::zero();
        }

        if let Some(result) = scene.find_intersection(ray) {
            let object = result.get_object();
            match object.get_type() {
                ObjectType::Solid(material) => {
                    let scattered_ray = material.scatter_ray(result.get_intersection_point(), result.get_intersection_normal(), rng);
                    let radiance = material.compute_radiance(result.get_intersection_normal(), scattered_ray.get_direction());
                    let pdf = material.pdf(result.get_intersection_normal(), scattered_ray.get_direction());

                    return self.compute_radiance_vec(scattered_ray, scene, depth + 1, rng) * radiance / pdf; 
                }

                ObjectType::Light(emission) => {
                    return Vec3::new(emission, emission, emission);
                }
            }
        }

        Vec3::zero()
    }
}

impl Integrator for PathTracerIntegrator {
    fn compute_radiance(&self, ray: Ray, scene: &Scene, rng: &mut XoroshiroRNG) -> Radiance {
        let radiance_vec = self.compute_radiance_vec(ray, scene, 0, rng);
        Radiance::new(radiance_vec.x(), radiance_vec.y(), radiance_vec.z())
    }
}
