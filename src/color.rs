use img;
use std::u8;

// A struct for all internal color management, but not for textures and objecs colors.
#[derive(Clone,Debug,Copy,Serialize,Deserialize,PartialEq)]
pub struct RGBA32 {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

// A struct to support the classic 8 bit color values that is used to : write to a .png file,
// manage object color through textures.
#[derive(Clone,Debug,Copy,Serialize,Deserialize)]
pub struct RGBA8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn u32_to_u8(v: u32) -> u8 {
    (v / (u32::max_value() / u8::max_value() as u32)) as u8
}

impl RGBA8 {
    // A fully opaque black color
    pub fn new_black() -> RGBA8 {
        RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: u8::max_value(),
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn new(r: &u8, g: &u8, b: &u8, a: &u8) -> RGBA8 {
        RGBA8 {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }
    }
}

impl RGBA32 {
    // A fully opaque black color
    pub fn new_black() -> RGBA32 {
        RGBA32 {
            r: 0,
            g: 0,
            b: 0,
            a: u32::max_value(),
        }
    }

    pub fn r(&self) -> u32 {
        self.r
    }

    pub fn g(&self) -> u32 {
        self.g
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn new(r: &u32, g: &u32, b: &u32, a: &u32) -> RGBA32 {
        RGBA32 {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }
    }
}

impl img::Pixel for RGBA32 {
    fn to_rgb_pixel(&self) -> (u8, u8, u8) {
        (u32_to_u8(self.r), u32_to_u8(self.g), u32_to_u8(self.b))
    }

    fn to_rgba_pixel(&self) -> (u8, u8, u8, u8) {
        (u32_to_u8(self.r), u32_to_u8(self.g), u32_to_u8(self.b), u32_to_u8(self.a))
    }
}
