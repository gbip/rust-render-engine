#[derive(Clone,Debug,Copy,Serialize,Deserialize)]
pub struct RGBA32 {
    r : u32,
    g : u32,
    b : u32,
    a : u32,
} 

impl RGBA32 {
    pub fn new_black() -> RGBA32 {
        RGBA32{r:0,g:0,b:0,a:0}
    }

    pub fn get_rgb(&self) -> (u32, u32, u32) {
        (self.r, self.g, self.b)
    }

    pub fn get_rgba(&self) -> (u32, u32, u32, u32) {
        (self.r, self.g, self.b, self.a)
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