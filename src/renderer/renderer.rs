use image::image::Image;
use image::pixel::{ColorChannelData, Pixel};
use renderer::integrator::{Radiance, Integrator};
use renderer::scene::scene::Scene;
use renderer::camera::Camera;
use util::xoroshiro_rng::XoroshiroRNG;

use term::loading_bar::LoadingBar;

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
        let mut loading_bar = LoadingBar::new(self.image_width as f32, 1.0);

        let mut image = Image::new(self.image_width, self.image_height);

        let mut rng = XoroshiroRNG::new();
        for row in 0..self.image_height {
            print!("\r{}", loading_bar.loading_bar_string());
            loading_bar.step();
            
            for col in 0..self.image_width {
                let mut pixel_radiance = Radiance::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let u = (col as f32 + rng.next_f32()) / self.image_width as f32;
                    let v = (row as f32 + rng.next_f32()) / self.image_height as f32;
                    let ray = camera.generate_ray(u, v);
                    let rad = self.integrator.compute_radiance(ray, scene, &mut rng);
                    pixel_radiance = pixel_radiance + rad;

                }

                pixel_radiance = pixel_radiance / self.samples_per_pixel as f32;
                pixel_radiance = self.correct_radiance(pixel_radiance);

                let red_channel = (pixel_radiance.red() * 255.99) as ColorChannelData;
                let green_channel = (pixel_radiance.green() * 255.99) as ColorChannelData;
                let blue_channel = (pixel_radiance.blue() * 255.99) as ColorChannelData;
                let pixel = Pixel::new(red_channel, green_channel, blue_channel);

                image.set_pixel(pixel, self.image_height - row, col);
            }
        }

        println!("");

        image
    }

    fn correct_radiance(&self, radiance: Radiance) -> Radiance {
        Radiance::new(radiance.red().abs().sqrt(), radiance.green().abs().sqrt(), radiance.blue().abs().sqrt())
    }
}
