use renderer::scene::scene::Scene;
use renderer::ray::Ray;

pub type RadiosityChannel = f32;

pub struct Radiosity {
    red: RadiosityChannel,
    green: RadiosityChannel,
    blue: RadiosityChannel,
}

impl Radiosity {
    pub fn new(red: RadiosityChannel, green: RadiosityChannel, blue: RadiosityChannel) -> Radiosity {
        Radiosity {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn get_red(&self) -> RadiosityChannel {
        self.red
    }

    pub fn get_green(&self) -> RadiosityChannel {
        self.green
    }

    pub fn get_blue(&self) -> RadiosityChannel {
        self.blue
    }
}

pub trait Integrator {
    // Keep notion of images separate from integrator...
    fn compute_radiosity(&self, ray: Ray) -> Radiosity;
}
