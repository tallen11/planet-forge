use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;
use threadpool::ThreadPool;

use image::image::Image;
use image::image_chunk::{ImageChunkDescriptor, ImageChunk};
use image::pixel::{ColorChannelData, Pixel};
use renderer::integrator::{Radiance, IntegratorType, Integrator};
use renderer::scene::scene::Scene;
use renderer::camera::Camera;
use renderer::camera::pinhole_camera::PinholeCamera;
use util::xoroshiro_rng::XoroshiroRNG;

use term::loading_bar::LoadingBar;

pub struct Renderer<'a> {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    camera: &'a Camera,
    scene: &'a Scene,
    integrator_type: IntegratorType,
    thread_count: u32,
}

impl<'a> Renderer<'a> {
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32, camera: &'a Camera, scene: &'a Scene, integrator_type: IntegratorType, thread_count: u32) -> Renderer<'a> {
        Renderer {
            image_width: image_width,
            image_height: image_height,
            samples_per_pixel: samples_per_pixel,
            camera: camera,
            scene: scene,
            integrator_type: integrator_type,
            thread_count: thread_count,
        }
    }

    pub fn render(&self) -> Image {
        let mut loading_bar = LoadingBar::new(self.image_width as f32, 1.0);

        let mut image = Image::new(self.image_width, self.image_height);
        let chunk_descriptors = image.chunkify(16, 16);
        
        let thread_pool = ThreadPool::new(self.thread_count as usize);

        let (tx, rx): (Sender<ImageChunk>, Receiver<ImageChunk>) = channel();
        for descriptor in chunk_descriptors.iter() {
            let tx_t = tx.clone();
            let desc_t = descriptor.clone();

            thread_pool.execute(move|| {
                let image_chunk = self.render_image_chunk(self.camera, self.scene, self.integrator_type, desc_t);
                tx_t.send(ImageChunk::from_descriptor(desc_t)).unwrap();
            });
        }

        let mut chunks: Vec<ImageChunk> = Vec::new();
        for _ in 0..chunk_descriptors.len() {
            match rx.recv() {
                Ok(image_chunk) => image.set_chunk(&image_chunk),
                Err(error) => println!("Thread Error: {}", error),
            }
        }

        // let (tx, rx) = channel();
        // for i in 0..chunk_descriptors.len() {
            // let tx = tx.clone();
            // let camera_clone = camera.clone();
            // let scene_clone = scene.clone();
            // let integrator_clone = self.integrator;
            // thread_pool.execute(move|| {
                // let cam = camera.clone();
                // let sc = scene.clone();
                // let int = self.integrator.as_ref();
                // let chunk = self.render_image_chunk(cam, sc, int, chunk_descriptors[i]);
                // tx.send(Box::new(chunk)).unwrap();
            // });
        // }


        // let mut rng = XoroshiroRNG::new();
        // for row in 0..self.image_height {
        //     print!("\r{}", loading_bar.loading_bar_string());
        //     loading_bar.step();
            
        //     for col in 0..self.image_width {
        //         let mut pixel_radiance = Radiance::new(0.0, 0.0, 0.0);
        //         for _ in 0..self.samples_per_pixel {
        //             let u = (col as f32 + rng.next_f32()) / self.image_width as f32;
        //             let v = (row as f32 + rng.next_f32()) / self.image_height as f32;
        //             let ray = camera.generate_ray(u, v);
        //             let rad = self.integrator.compute_radiance(ray, scene, &mut rng);
        //             pixel_radiance = pixel_radiance + rad;

        //         }

        //         pixel_radiance = pixel_radiance / self.samples_per_pixel as f32;
        //         pixel_radiance = self.correct_radiance(pixel_radiance);

        //         let red_channel = (pixel_radiance.red() * 255.99) as ColorChannelData;
        //         let green_channel = (pixel_radiance.green() * 255.99) as ColorChannelData;
        //         let blue_channel = (pixel_radiance.blue() * 255.99) as ColorChannelData;
        //         let pixel = Pixel::new(red_channel, green_channel, blue_channel);

        //         image.set_pixel(pixel, self.image_height - row, col);
        //     }
        // }

        // println!("");

        image
    }

    fn render_image_chunk(&self, camera: &'a Camera, scene: &'a Scene, integrator_type: IntegratorType, descriptor: ImageChunkDescriptor) -> ImageChunk {
        let integrator = integrator_type.instantiate();

        let chunk = ImageChunk::from_descriptor(descriptor);
        chunk
    }

    fn correct_radiance(&self, radiance: Radiance) -> Radiance {
        Radiance::new(radiance.red().abs().sqrt(), radiance.green().abs().sqrt(), radiance.blue().abs().sqrt())
    }
}
