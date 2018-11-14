use image::image::Image;
use renderer::integrator::Integrator;
use renderer::scene::scene::Scene;
use renderer::camera::Camera;
use util::xoroshiro_rng::XoroshiroRNG;

pub struct Renderer {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    integrator: Box<Integrator>,
}

impl Renderer {
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32, integrator: Box<Integrator>) -> Renderer {
        Renderer {
            image_width: image_width,
            image_height: image_height,
            samples_per_pixel: samples_per_pixel,
            integrator: integrator,
        }
    }

    pub fn render(&self, camera: &Camera, scene: &Scene) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        // TODO: Don't hardcode seed forever...
        let mut rng = XoroshiroRNG::new([0x5565b7c1, 0x90bcecde, 0x3aaba6bf, 0xee9e95f4]);
        for row in (0..self.image_height).rev() {
            for col in 0..self.image_width {
                
                for _ in 0..self.samples_per_pixel {
                    let u = (col as f32 + rng.next_f32()) / self.image_width as f32;
                    let v = (row as f32 + rng.next_f32()) / self.image_height as f32;
                    let ray = camera.generate_ray(u, v);
                    let rad = self.integrator.compute_radiance(ray, scene);
                }
            }
        }

        image
    }
}
