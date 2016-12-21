use std::vec::Vec;
mod math;

type Vector3f = math::Vector3<f32>;

mod obj_3d {
    use math;
    struct Triangle<'a>{
        vertex : [&'a math::Vector3<f32>; 3]
    }

    struct Mesh<'a> {
        vertex_list : Vec<math::Vector3<f32>>,
        triangle_list : Vec<Triangle<'a>>,
    }
}

mod scene {
    use math::{Vector3, VectorialOperations};
    pub struct Camera {
    
        world_position : Vector3<f32>, // The position of the camera exprimmed in the standard world space 
        
        target_position : Vector3<f32>, // The position of the point at which the camera is aiming

        base_vector : [Vector3<f32>; 3], // The vectors that defines a new base where k1 is the direction in which the camera is aiming. All the vectors are forming an orthnormal base of R3.



    }

    impl Camera {
        fn init_cam(self : &mut Camera, cam_position : Vector3<f32>, target_position : Vector3<f32> ) {
            let v1 = cam_position - target_position;// First vector of the new base 
            let k1 = &v1/v1.norm_ref(); // k1 is the normed vector
        }

    }
}


fn main() {
    println!("Hello, world!");
}
