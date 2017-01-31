use std::vec::Vec;
use scene;

extern crate image;

/// A tuple that represents a color in a RGB value on 8 bits
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Color8 {
    r : u8,
    g : u8,
    b : u8,
}

impl Color8 {
    fn is_black(&self) -> bool {
        self.r==0 && self.g == 0 && self.b == 0
    }
    pub fn new_black() -> Self {
        Color8{r:0,g:0,b:0}
    }
    pub fn new(r:u8,g:u8,b:u8) -> Self {
        Color8{r:r,g:g,b:b}
    }
}

pub struct ImageData<T> {
    pub pixels : Vec<Vec<T>>
}

pub type Image = ImageData<Color8>;

impl ImageData<Color8> {
    fn display_black(&self) {
        let mut result : String = "".to_string();
            for line in self.pixels.iter() {
                for pixel in line.iter() {
                    if !pixel.is_black() {
                        result.push('#');
                    }
                    else {
                        result.push('*');
                    }
                }
                result.push('\n');
            }
            println!("{}",result);
    }

    pub fn draw_horizontal_line(&mut self,a:usize,b:usize,y:usize, color:Color8) {
        for i in a..b {
            self.pixels[y][i] = color.clone();
        }
    }
    pub fn new(sizex:usize,sizey:usize) -> Self {
        let px : Vec<Vec<Color8>> = vec!(vec!(Color8::new_black()));
        Image{pixels:px}
    }
}

use math::Vector2;
/// Represent a Surface on which you can draw a picture (a screen, a file, etc.)
trait Surface {
    fn write_to(data : &Image);
}

/// An internal data structur that represent the boundary box of the region to be rendered
/// where u is the bottom left corner and v is the top right corner
pub struct Canvas {
    pub u : Vector2<f32>,
    pub v : Vector2<f32>,
}

/// Represents a window through which the camera is seeing the world. It depends mainly of the
/// FOV of the camera, and the clipping distance between the virtual point of the camera and
/// this window
impl Canvas {

    /// This method will panic if the canvas is a line or a dot: u1.x = u2.x or u1.y = u2.y . It
    /// will also panic if u1.x > u2.x or u1.y > u2.y
    /// TODO : Make this return a result and not panic!()...
    pub fn new(u1: Vector2<f32>, u2: Vector2<f32>) -> Canvas {
       assert!(u1.x != u2.x && u1.y != u2.y);
       assert!(u1.x < u2.x && u1.y < u2.y);
       Canvas{u:u1,v:u2}
    }

    /// Returns the 4 corner of the rendering rectangle, in a clockwise order, starting by the
    /// top left corner.
    pub fn get_corners(&self) -> [Vector2<f32>;4] {
        let u1 = Vector2{x: self.u.x,y: self.v.y};
        let u2 = Vector2{x: self.v.x,y: self.v.y};
        let u3 = Vector2{x: self.v.x,y: self.u.x};
        let u4 = Vector2{x: self.u.x,y: self.u.y};
        [u1,u2,u3,u4]
    }
}
