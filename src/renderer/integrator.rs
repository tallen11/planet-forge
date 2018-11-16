use renderer::scene::scene::Scene;
use renderer::ray::Ray;
use util::xoroshiro_rng::XoroshiroRNG;
use renderer::path_tracer_integrator::PathTracerIntegrator;

pub type RadianceChannel = f32;

#[derive(Copy, Clone, Debug)]
pub struct Radiance {
    red: RadianceChannel,
    green: RadianceChannel,
    blue: RadianceChannel,
}

impl Radiance {
    pub fn new(red: RadianceChannel, green: RadianceChannel, blue: RadianceChannel) -> Radiance {
        Radiance {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn red(&self) -> RadianceChannel {
        self.red
    }

    pub fn green(&self) -> RadianceChannel {
        self.green
    }

    pub fn blue(&self) -> RadianceChannel {
        self.blue
    }
}

impl std::ops::Add<Radiance> for Radiance {
    type Output = Radiance;

    fn add(self, _rhs: Radiance) -> Radiance {
        Radiance {
            red: self.red + _rhs.red(),
            green: self.green + _rhs.green(),
            blue: self.blue + _rhs.blue(),
        }
    }
}

impl std::ops::Div<f32> for Radiance {
    type Output = Radiance;

    fn div(self, _rhs: f32) -> Radiance {
        Radiance {
            red: self.red / _rhs,
            green: self.green / _rhs,
            blue: self.blue / _rhs,
        }
    }
}

pub enum IntegratorType {
    PathTracer(u32),
}

impl IntegratorType {
    pub fn instantiate(&self) -> Box<Integrator> {
        match *self {
            IntegratorType::PathTracer(max_ray_bounces) => Box::new(PathTracerIntegrator::new(max_ray_bounces)),
            _ => unreachable!(),
        }
    }
}

pub trait Integrator {
    fn compute_radiance(&self, ray: Ray, scene: &Scene, rng: &mut XoroshiroRNG) -> Radiance;
}
