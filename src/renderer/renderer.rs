use renderer::integrator::Integrator;
use image::image::Image;

pub struct Renderer {
    image_width: u32,
    image_height: u32,
    integrator: Box<Integrator>,
}

impl Renderer {
    pub fn new(image_width: u32, image_height: u32, integrator: Box<Integrator>) -> Renderer {
        Renderer {
            image_width: image_width,
            image_height: image_height,
            integrator: integrator,
        }
    }

    pub fn render(&self) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        

        image
    }
}
