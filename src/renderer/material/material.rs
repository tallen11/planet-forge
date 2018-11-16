use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;
use renderer::ray::Ray;
use util::xoroshiro_rng::XoroshiroRNG;
use util::sample_unit_hemisphere_uniform;

pub const SCATTER_EPSILON: f32 = 1.0e-4;

pub trait Material {
    fn scatter_ray(&self, intersection_point: Point, surface_normal: Vec3, rng: &mut XoroshiroRNG) -> Ray;
    fn compute_radiance(&self, surface_normal: Vec3, incident: Vec3) -> Vec3;
    fn brdf(&self, surface_normal: Vec3, incident: Vec3) -> Vec3;
    fn pdf(&self, surface_normal: Vec3, incident: Vec3) -> f32;
    // fn radiance(&self, source_intensity: f32, surface_normal: Vec3, incident: Vec3) -> Vec3;
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
        // let rand = rng.next_f32();
        // let r = rand.sqrt() * 

        let n_z = Vec3::new(surface_normal.z(), 0.0, -surface_normal.x()).normalized();
        let n_b = n_z.cross(surface_normal).normalized();
        
        let sample = sample_unit_hemisphere_uniform(rng);
        let scattered_direction = Vec3::new(
            sample.x()*n_b.x() + sample.y()*surface_normal.x() + sample.z()*n_z.x(),
            sample.x()*n_b.y() + sample.y()*surface_normal.y() + sample.z()*n_z.y(),
            sample.x()*n_b.z() + sample.y()*surface_normal.z() + sample.z()*n_z.z()).normalized();

        Ray::new(intersection_point + (SCATTER_EPSILON * scattered_direction).to_point(), scattered_direction)
    }

    fn compute_radiance(&self, surface_normal: Vec3, incident: Vec3) -> Vec3 {
        self.albedo * (1.0 / std::f32::consts::PI) * incident.dot(surface_normal)
    }

    fn brdf(&self, surface_normal: Vec3, incident: Vec3) -> Vec3 {
        self.albedo * (1.0 / std::f32::consts::PI) * surface_normal.dot(incident)
    }

    fn pdf(&self, surface_normal: Vec3, incident: Vec3) -> f32 {
        incident.dot(surface_normal) / std::f32::consts::PI
    }
}

// TODO: Oren-Nayar model is ideally suited to simulate the appearance of the moon which is not reflecting light exactly light a diffuse surface would.

