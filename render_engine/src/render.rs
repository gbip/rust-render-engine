use std::vec::Vec;
use math;
use scene;
use std;
use obj3D::IsPolygon;

/// A tuple that represents a color in a RGB value on 8 bits
pub type Color8=(u8,u8,u8);

    type Image = Vec<Vec<Color8>> ;//[Box<[Color8]>]; 
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

    impl Canvas {

        /// This method will panic if the canvas is a line or a dot: u1.x = u2.x or u1.y = u2.y . It
        /// will also panic if u1.x > u2.x or u1.y > u2.y
        
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
        
    
    

    
    /// Represent a rendering algorithm
    trait Render {
        fn generate_image(world: scene::World) -> Image;
    }

    // TODO : To implement
    fn z_buffer_renderer(world :& scene::World, sizeX : usize, sizeY : usize) -> Image {
        let z_array = vec![vec![ std::f64::INFINITY; sizeY]; sizeX];
        let result =  vec![vec![ (0_u8,0_u8,0_u8); sizeY]; sizeX];
        
        for object in &world.objects {
            for triangle in object.get_triangles() {
                /*let polygon = triangle.trim_to_window(sizeX as u64, sizeY as u64);
                let pixels = polygon.find_points_in_polygon(sizeX as u64, sizeY as u64);
                for pixel in pixels {
                */
                //}
            }
        }

        result
    }
