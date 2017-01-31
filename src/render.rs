use std::vec::Vec;
use std::fs::File;
use std::path::Path;
use scene;

use image;

/// A tuple that represents a color in a RGB value on 8 bits
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Color8 {
    r : u8,
    g : u8,
    b : u8,
}

pub struct ImageData<T : Color> {
    width : usize,
    height : usize,
    pub pixels : Vec<T>
}

pub trait Color : Clone {
    fn new_neutral() -> Self;
    fn get_rgb(&self) -> (u8, u8, u8);
    fn get_rgba(&self) -> (u8, u8, u8, u8);
}


impl Color8 {
    pub fn new(r:u8,g:u8,b:u8) -> Self {
        Color8{r:r,g:g,b:b}
    }
}

impl Color for Color8 {
    fn new_neutral() -> Self {
        Color8{r:0,g:0,b:0}
    }

    fn get_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn get_rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, 255_u8)
    }
}

impl<T : Color> ImageData<T> {

    pub fn new(sizex:usize, sizey:usize) -> Self {
        let px : Vec<T> = vec![T::new_neutral(); sizex * sizey];
        ImageData::<T> {width : sizex, height: sizey, pixels:px}
    }

    pub fn write_to_file(&self, pathname : &str) {
        let mut buffer = vec!();

        for ref color in self.pixels.iter() {
            let (r, g, b) = color.get_rgb();
            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
        }

        let _ = image::save_buffer(&Path::new(pathname), buffer.as_slice(), self.width as u32, self.height as u32, image::RGB(8)).unwrap();
    }
}

use math::Vector2;

pub struct Renderer {
    res_x : usize,
    res_y : usize,
}

impl Renderer {
    pub fn new(res_x : usize, res_y : usize) -> Self {
        Renderer {res_x : res_x, res_y : res_y}
    }

    pub fn render(&self, world : &scene::World, camera : &mut scene::Camera) -> ImageData<Color8> {
        // Création de l'image qui résulte du rendu
        let mut result = ImageData::<Color8>::new(self.res_x, self.res_y);

        // On paramètre la caméra
        let fres_x = self.res_x as f32;
        let fres_y = self.res_y as f32;
        camera.ratio = fres_y / fres_x;

        // On crée les "canvas"

        // On emet les rayons

        // Post process

        // Chaque pixel est recomposé suivant les rayons qui en ont été émis

        result
    }
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
