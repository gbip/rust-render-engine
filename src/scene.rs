use std::vec::Vec;
use math::{Vector3,VectorialOperations, Vector3f};
use obj3D;
use obj3D::Object;
use std::fs::File;
use std::io::{Write};
use std;
use serde_json;
use render::Color8;

fn write_string_to_file(j:&str,file_name:String) -> std::io::Result<()> {
        let mut file = File::create(file_name).unwrap();
            file.write_all(j.as_bytes())
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
pub struct R3Base{
    u: Vector3f,
    v: Vector3f,
    w: Vector3f
}

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

#[derive(Serialize,Deserialize)]
pub struct World<'a> {
    /// The base vector of the world :
    /// the 3rd one is UP (aka we are in XYZ configuration)
    base_vector : [Vector3<f32>; 3],
    /// A Vec containing all the cameras in the world
    cameras : Vec<Camera>,

    objects : Vec<obj3D::Object<'a>>,
}

impl<'a> World<'a> {
    /// This method create a camera at <position>, aiming at <target>
    fn add_camera(self : &'a mut World<'a>, position : Vector3f, target : Vector3f) {
        let mut cam_base = R3Base{u: Vector3::new(0_f32,0_f32,0_f32),
                                  v: Vector3::new(0_f32,0_f32,0_f32),
                                  w: Vector3::new(0_f32,0_f32,0_f32)};
        cam_base.make_camera_base(&position,&target,&self.base_vector[2]);
        let new_cam = Camera{world_position: position,
                             target_position: target,
                             base_vector: cam_base};
        self.cameras.push(new_cam);
    }

    /// Load all objects meshes
    pub fn load_objects(&'a mut self) {
        self.objects.iter_mut().map(|obj| obj.load_mesh());
    }

    //Generates a new empty world
    pub fn new_empty() -> World<'a> {
        let base_vector = [Vector3::new(1_f32,0_f32,0_f32),Vector3::new(0_f32,1_f32,0_f32),Vector3::new(0_f32,0_f32,1_f32)];
        World{base_vector:base_vector,
              cameras:vec!(),
              objects:vec!()}
    }

    //Allocates a new object so that we can initialize it
    fn get_new_empty_object(&'a mut self) -> &'a mut Object {
        self.objects.push(Object::new_empty());
        self.objects.last_mut().unwrap()
    }


    //Add an object to the world
    pub fn add_object(&'a mut self,color:Color8,pos:Vector3f,path:String) {

        Object::initialize(self.get_new_empty_object(),color,pos,path);

    }

    pub fn save_world_to_file(&self,file:&str) {
        match write_string_to_file(&serde_json::to_string(&self).unwrap() ,file.to_string()) {

            Err(e) =>println!("Could not save world. Error : {}",e),

            Ok(_) =>println!("World sucessfully saved"),

        }
    }
    pub fn load_world_from_file<'b>(file: String) -> World<'b> {
       let world : World = serde_json::from_str(file.as_str()).unwrap();
        world
    }
}
