pub type ColorChannelData = u8;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    red: ColorChannelData,
    green: ColorChannelData,
    blue: ColorChannelData,
}

impl Pixel {
    pub fn new(red: ColorChannelData, green: ColorChannelData, blue: ColorChannelData) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn black() -> Pixel {
        Pixel {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn get_red(&self) -> ColorChannelData {
        self.red
    }

    pub fn get_green(&self) -> ColorChannelData {
        self.green
    }

    pub fn get_blue(&self) -> ColorChannelData {
        self.blue
    }
}