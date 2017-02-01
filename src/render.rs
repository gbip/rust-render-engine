
use scene;
use img::Image;
use color::RGBA32;

use math::Vector2;

pub struct Renderer {
    res_x : usize,
    res_y : usize,
    ratio : f32,
}

impl Renderer {
    pub fn new(res_x : usize, res_y : usize) -> Self {
        Renderer {res_x : res_x,
            res_y : res_y,
            ratio : (res_x as f32/res_y as f32)}
    }

    pub fn render(&self, world : &scene::World, camera : &mut scene::Camera) -> Image<RGBA32> {
        // Création de l'image qui résulte du rendu
        let result = Image::<RGBA32>::new(self.res_x, self.res_y);

        // On crée les "canvas"

        // On emet les rayons

        // Post process

        // Chaque pixel est recomposé suivant les rayons qui en ont été émis

        //result;
        unimplemented!();
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
