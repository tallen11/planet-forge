use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;
use renderer::ray::Ray;
use util::xoroshiro_rng::XoroshiroRNG;
use util::sample_unit_hemisphere_uniform;

pub trait Material {
    fn scatter_ray(&self, intersection_point: Point, surface_normal: Vec3, rng: &mut XoroshiroRNG) -> Ray;
    fn brdf(&self, surface_normal: Vec3, incident: Vec3) -> Vec3;
    fn radiance(&self, source_intensity: f32, surface_normal: Vec3, incident: Vec3) -> Vec3;
}

pub struct LambertianMaterial {
    albedo: Vec3,
}

impl LambertianMaterial {
    pub fn new(albedo: Vec3) -> LambertianMaterial {
        LambertianMaterial {
            albedo: albedo,
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter_ray(&self, intersection_point: Point, surface_normal: Vec3, rng: &mut XoroshiroRNG) -> Ray {
        let target = intersection_point + surface_normal + sample_unit_hemisphere_uniform(rng);
        let scattered = Ray::new(intersection_point, target - intersection_point);
        scattered
    }

    fn brdf(&self, surface_normal: Vec3, incident: Vec3) -> Vec3 {
        self.albedo * (1.0 / std::f32::consts::PI)
    }

    fn radiance(&self, source_intensity: f32, surface_normal: Vec3, incident: Vec3) -> Vec3 {
        self.brdf(surface_normal, incident) * source_intensity * surface_normal.dot(incident)
    }
}

// TODO: Oren-Nayar model is ideally suited to simulate the appearance of the moon which is not reflecting light exactly light a diffuse surface would.

