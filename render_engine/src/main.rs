use std::vec::Vec;

// Implementation of the Indexed Face Set data structure for representing a mesh
mod math {
    use std::ops::{Add,Sub,Mul};
    pub struct Vector3<T> {
        pub x : T,
        pub y : T,
        pub z : T,
    }

    impl<T> Add<Vector3<T>> for Vector3<T>
        where T: Add<Output=T> {
        type Output = Vector3<T>;

        fn add(self, other: Vector3<T>) -> Vector3<T> {

            Vector3{x : self.x + other.x, y : self.y + other.y, z: self.z + other.z}  
        }
    }

    impl<T> Sub<Vector3<T>> for Vector3<T>
        where T: Sub<Output=T> {
        type Output = Vector3<T>;

        fn sub(self, other: Vector3<T>) -> Vector3<T> {

            Vector3{x : self.x - other.x, y : self.y - other.y, z: self.z - other.z}  
        }
    }

    trait VectorialOperations<T> {
    
        fn norm(self) -> f32;
        
        fn cross_product(self, other : Vector3<T>) -> Vector3<T>;
    }

    impl<T> VectorialOperations<T> for Vector3<T> where
    T : Copy + Mul<Output=T> + Add<Output=T> + Into<f32> + Sub<Output=T>,
        {
        fn norm(self) -> f32 {
        ( (self.x*self.x) + (self.y*self.y) + (self.z*self.z) ).into()
        }
        fn cross_product(self, other : Vector3<T>) -> Vector3<T> {
            Vector3{x : self.y * other.z - self.z * other.y, 
                    y : self.z * other.x - self.x * other.z,
                    z : self.x * other.y - self.y * other.x}
    
        }
    }
}


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
    use math::Vector3;
    pub struct Camera {
    
        world_position : Vector3<f32>, //The position of the camera exprimmed in the standard world space 
        
        target_position : Vector3<f32>, //The position of the point at which the camera is aiming

        base_vector : [Vector3<f32>; 3], //The vectors that defines a new base



    }

    impl Camera {
        fn init_cam(self : &mut Camera, cam_position : Vector3<f32>, target_position : Vector3<f32> ) {
            let k1 = cam_position - target_position; //First vector of the new base 
        }

    }
}


fn main() {
    println!("Hello, world!");
}
