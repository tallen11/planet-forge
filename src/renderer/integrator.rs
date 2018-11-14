use renderer::scene::scene::Scene;
use renderer::ray::Ray;

pub type RadianceChannel = f32;

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

    pub fn get_red(&self) -> RadianceChannel {
        self.red
    }

    pub fn get_green(&self) -> RadianceChannel {
        self.green
    }

    pub fn get_blue(&self) -> RadianceChannel {
        self.blue
    }
}

pub trait Integrator {
    fn compute_radiance(&self, ray: Ray, scene: &Scene) -> Radiance;
}
