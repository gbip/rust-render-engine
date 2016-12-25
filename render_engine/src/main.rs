use std::vec::Vec;
#[macro_use]
mod math;

type Vector3f = math::Vector3<f32>;
/// A tuple that represents a color in a RGB value on 8 bits
type Color8=(u8,u8,u8);

mod obj_3d {
    use math;
   
    /// This structs only hold references to the vertex that are stocked in the mesh.
    struct Triangle<'a>{
        vertex : [&'a math::Vector3<f32>; 3]
    }
    
    struct Polygon<'a> {
    vertex : Vec<&'a math::Vector3<f32>>
    }

    trait IsPolygon<'a> {
        fn find_points_in_polygon(&'a self, sizeX : u64, sizeY : u64) -> Vec<math::Vector2<u16>>;
    }

    impl<'a> IsPolygon<'a> for Polygon<'a> {
        fn find_points_in_polygon(&'a self, sizeX : u64, sizeY : u64) -> Vec<math::Vector2<u16>>
        {
        vec!()
        }
    }

    
    impl<'a> Triangle<'a> {
        fn trim_to_window(self, sizeX : u64, sizeY : u64) -> Polygon<'a> {
        Polygon{vertex: vec!()}
        }
    }

    
    /// The standard Indexed Face Set data structure for mesh.
    pub struct Mesh<'a> {
        vertex_list : Vec<math::Vector3<f32>>,
        triangle_list : Vec<Triangle<'a>>,
    }
}

mod renderer {
    use Color8; 
    use scene;
    
    type Image = Vec<Vec<Color8>> ;//[Box<[Color8]>]; 
    
    /// Represent a Surface on which you can draw a picture (a screen, a file, etc.)
    trait Surface {
        fn write_to(data : &Image);
    }
    
    /// Represent a rendering algorithm
    trait Render {
        fn generate_image(world: scene::World) -> Image;
    }

    // TODO : To implement
    fn z_buffer_renderer(world :& scene::World) -> Image {
        
        vec!(vec!((0_u8,0_u8,0_u8)))

    }
}

mod scene {
    use math::{Vector3, VectorialOperations};
    use Vector3f;
    use obj_3d;
    
    pub struct Camera {
   
        /// The position fo the camera exprimed in the standard word space coordinates (where {0,0,0} is the
        /// center of the world)
        world_position : Vector3f,  
       
        /// The position of the point at which the camera is aiming, in world space coordinates
        target_position : Vector3f, 
        
        /// This array of vector is defining a new orthnormal base of the space where :
        ///     - k1 is the vector that describes how the camera is aiming (ie
        /// world_position-target_position).
        ///     - k2 and k3 are choosen randomly.
        base_vector : R3Base, 
    }

    impl Camera {

        fn project_into_camera_base(vector :Vector3f) -> Vector3f {

            Vector3::make_vec3(0_f32,0_f32,0_f32)
        }

    }
    
    pub struct R3Base{
       u: Vector3f, 
        v: Vector3f,
        w: Vector3f}

    impl R3Base {
        
        fn make_camera_base<'a>(&mut self, camera_position : &'a Vector3f, target_position : &'a Vector3f, world_z_axis : &Vector3f) {
            /// Initialisation of the base vectors
            let v1 = camera_position - target_position;// First vector of the new base 
            let k1 = &v1/v1.norm_ref(); // k1 is the normed vector

            let v2 = k1.cross_product_ref(world_z_axis);
            let k2 = &v2/v2.norm_ref();

            let v3 = k1.cross_product_ref(&k2); // Since we need an orthonormal base, there isn't many choice for this one.
            let k3 = &v3/v3.norm();
            
            // TODO : Make some pattern matching here
            self.u= k1;
            self.v= k2;
            self.w= k3;
            
        }
    }

    pub struct World<'a> {
        /// The base vector of the world :
        /// the 3rd one is UP (aka we are in XYZ configuration)
        base_vector : [Vector3<f32>; 3],
       
        /// A Vec containing all the cameras in the world
        cameras : Vec<Camera>,
    
        objects : Vec<obj_3d::Mesh<'a>>,
    }

    impl<'a> World<'a> {
        /// This method create a camera at <position>, aiming at <target>
        fn add_camera(self : &mut World<'a>, position : Vector3f, target : Vector3f) {
            let mut cam_base = R3Base{u: Vector3::make_vec3(0_f32,0_f32,0_f32),
                                      v: Vector3::make_vec3(0_f32,0_f32,0_f32),
                                      w: Vector3::make_vec3(0_f32,0_f32,0_f32)};
            cam_base.make_camera_base(&position,&target,&self.base_vector[2]);
            let new_cam = Camera{world_position: position,
                                 target_position: target,
                                 base_vector: cam_base};
            self.cameras.push(new_cam);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
