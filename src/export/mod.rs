pub mod png_exporter;

use image::ColorChannelData;

pub trait Exporter {
    fn export_image_data_to_file(image_data: &Vec<ColorChannelData>, image_width: u32, image_height: u32, path_str: &str) -> std::io::Result<()>;
}
