use img;
use std::u8;

#[derive(Clone,Debug,Copy,Serialize,Deserialize)]
pub struct RGBA32 {
    r : u32,
    g : u32,
    b : u32,
    a : u32,
}

fn u32_to_u8(v : u32) -> u8 {
    (v / (u32::max_value() / u8::max_value() as u32)) as u8
}

impl RGBA32 {
    pub fn new_black() -> RGBA32 {
        RGBA32{r:0,g:0,b:0,a:0}
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

    pub fn new(r:&u32,g:&u32,b:&u32,a:&u32) -> RGBA32 {
        RGBA32{r:*r,g:*g,b:*b,a:*a}
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
