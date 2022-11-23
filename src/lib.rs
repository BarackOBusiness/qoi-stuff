pub enum Channels {
    RGB = 3,
    RGBA = 4,
}

pub enum ColorSpace {
    SRGB = 0,
    Linear = 1,
}

struct QoiHeader {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

impl QoiHeader {
    pub fn qoi_write(filename: char) {
        
    }
}