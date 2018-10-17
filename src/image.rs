pub type PixelValue = u8;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: PixelValue,
    pub green: PixelValue,
    pub blue: PixelValue,
    pub alpha: PixelValue,
}

impl Pixel {
    pub fn new(red: PixelValue, green: PixelValue, blue: PixelValue) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
            alpha: 255,
        }
    }
}

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            data: vec![Pixel::new(0, 0, 0); width*height],
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: Pixel) {
        let i = col + row * self.width;
        assert!(i < self.width*self.height);

        self.data[i] = pixel;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn export_to_raw(&self) -> Vec<PixelValue> {
        self.data
                .iter()
                .flat_map(|pixel| vec![pixel.red, pixel.green, pixel.blue, pixel.alpha])
                .collect()
    }
}
