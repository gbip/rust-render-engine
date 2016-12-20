use std::vec::Vec;

// Implementation of the Indexed Face Set data structure for representing a mesh
mod math {
    pub struct Vector3<T> {
        pub x : T,
        pub y : T,
        pub z : T,
    }
}

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





fn main() {
    println!("Hello, world!");
}
